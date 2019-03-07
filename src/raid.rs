use rand::prelude::*;

static BLOCKSIZE:i32 = 4096;

fn convert (size:String) -> i32 {
    let length = size.len();
    let lastchar = &size[length-1..length];
    let mut nsize = 0;
    if  lastchar == "k" || lastchar == "K" {
        let m = 1024;
        nsize = &size[0..length-1].parse().unwrap() * m;
    } else if lastchar == "m" || lastchar == "M" {
        let m = 1024*1024;
        nsize = &size[0..length-1].parse().unwrap() * m;
    }else if lastchar == "g" || lastchar == "G" {
        let m = 1024*1024*1024;
        nsize = &size[0..length-1].parse().unwrap() * m;
    } else {
        nsize =  size.parse().unwrap();
    }
    nsize
}

struct Disk {
    seekTime:i32,
    xferTime:f64,
    // length of scheduling queue
    queueLen:i32,
    currAddr:i32,
    //queue:Vec<>,

    // disk geometry
    numTracks:i32,
    blocksPerTrack:i32,
    blocksPerDisk:i32,

    // stats
    countIO:i32,
    countSeq:i32,
    countNseq:i32,
    countRand:i32,
    utilTime:f64,
}

impl Disk {
    fn new() -> Disk{
        Disk {
            seekTime:10,
            xferTime:0.1,
            // length of scheduling queue
            queueLen:8,
            currAddr:-10000,
            //queue:Vec<>,

            // disk geometry
            numTracks:100,
            blocksPerTrack:100,
            blocksPerDisk:10000,

            // stats
            countIO:0,
            countSeq:0,
            countNseq:0,
            countRand:0,
            utilTime:0.0,
        }
    }
    fn new_a(seekTime:i32,xferTime:f64,queueLen:i32) -> Disk{
        Disk {
            seekTime:seekTime,
            xferTime:xferTime,
            // length of scheduling queue
            queueLen:queueLen,
            currAddr:-10000,
            //queue:Vec<>,

            // disk geometry
            numTracks:100,
            blocksPerTrack:100,
            blocksPerDisk:10000,

            // stats
            countIO:0,
            countSeq:0,
            countNseq:0,
            countRand:0,
            utilTime:0.0,
        }
    }

    fn stats(&self) ->(i32,i32,i32,i32,f64){
        return (self.countIO,self.countSeq,self.countNseq,self.countRand,self.utilTime);
    }

    fn enqueue(&mut self,addr:i32) {
        assert!(addr< self.blocksPerDisk);

        self.countIO += 1;

        let currTrack = self.currAddr / self.numTracks;

        let newTrack = addr /self.numTracks;

        let diff = addr - self.currAddr;

        if currTrack == newTrack || diff < self.blocksPerTrack {
            if diff == 1 {
                self.countSeq += 1;
            }else {
                self.countNseq += 1;
            }
            self.utilTime += diff as f64 * self.xferTime;
        }else {
            self.countRand += 1;
            self.utilTime += self.seekTime as f64 + self.xferTime;
        }

        self.currAddr = addr;
    }

    fn go (&self) -> f64 {
        return self.utilTime;
    }
}

struct Raid {
    chunkSize:i32,
    numDisks:i32,
    raidLevel:i32,
    timing:bool,
    reverse:bool,
    solve:bool,
    raid5type:String,
    disks:Vec<Disk>,
    blocksInStripe:i32,
    pdisk:i32,
    printPhysical:bool,
}

impl Raid{
    fn new(chunkSize:i32,numDisks:i32,level:i32,timing:bool,reverse:bool,solve:bool,raid5type:String) -> Raid {
        Raid {
            chunkSize:chunkSize,
            numDisks:numDisks,
            raidLevel:level,
            timing:timing,
            reverse:reverse,
            solve:solve,
            raid5type:raid5type,
            disks:Vec::new(),
            blocksInStripe:0,
            pdisk:0,
            printPhysical:false,
        }
    }

    fn init(&mut self) {
        if (self.chunkSize & BLOCKSIZE) != 0 {
            println!("chunksize ({}) must be multiple of blocksize ({}): {}" ,self.chunkSize, BLOCKSIZE, self.chunkSize % BLOCKSIZE);
        }

        if self.raidLevel == 1 && self.numDisks & 2 != 0 {
            println!("raid1: disks ({}) must be a multiple of two" , self.numDisks);
        }

        if self.raidLevel == 4 {
            self.blocksInStripe = (self.numDisks - 1) * self.chunkSize;
            self.pdisk = self.numDisks - 1;
        }

        if self.raidLevel ==5 {
            self.blocksInStripe = (self.numDisks - 1) * self.chunkSize;
            self.pdisk = -1;
        }

        for i in 0..self.numDisks {
            self.disks.push(Disk::new());
        }

    }

    fn stats(&self ,totalTime:f64) {
        for d in 0..self.numDisks {
            let s = self.disks[d as usize].stats();
            if s.4 == totalTime {
                println!("disk:{}  busy: {}  I/Os: {} (sequential:{} nearly:{} random:{})",  d, (100.0f64*s.4/totalTime), s.0, s.1, s.2, s.3);
            }else if s.4 == 0.0 {
                println!("disk:{}  busy: {}  I/Os: {} (sequential:{} nearly:{} random:{})",  d, (100.0f64*s.4/totalTime), s.0, s.1, s.2, s.3);
            }else {
                println!("disk:{}  busy: {}  I/Os: {} (sequential:{} nearly:{} random:{})",  d, (100.0f64*s.4/totalTime), s.0, s.1, s.2, s.3);
            }
        }
    }

    fn enqueue(&mut self,addr:i32,size:i32,isWrite:bool) {
        if self.timing == false {
            if self.solve || self.reverse == false {
                if isWrite {
                    println!("LOGICAL WRITE to  addr:{} size:{}" ,addr, size * BLOCKSIZE);
                }else {
                    println!("LOGICAL READ to  addr:{} size:{}" ,addr, size * BLOCKSIZE);
                }

                if self.solve {
                    println! ("  Physical reads/writes?");
                }
            }else {
                 println! ("LOGICAL OPERATION is ?");
            }
        }

        if self.timing == false && (self.solve || self.reverse==true){
            self.printPhysical = true;
        }else{
            self.printPhysical = false;
        }

        if self.raidLevel == 0{
            self.enqueue0(addr, size, isWrite);
        } else if self.raidLevel == 1{
            self.enqueue1(addr, size, isWrite);
        } else if self.raidLevel == 4 || self.raidLevel == 5{
            self.enqueue45(addr, size, isWrite);
        }
    }

    fn go (&self) ->f64{
        let mut tmax = 0.0;
        for d in 0..self.numDisks {
            let t = self.disks[d as usize].go();
            if t>tmax {
                tmax = t;
            }
        }
        return tmax;
    }

    // helper functions

    fn doSingleRead(&mut self,disk:i32,off:i32,doNewline:bool) {
        if self.printPhysical {
            println!("  read  [disk {}, offset {}]  ",disk,off);
            if doNewline {
                println!("");
            }
        }
        self.disks[disk as usize].enqueue(off);
    }

    fn doSingleWrite(&mut self,disk:i32,off:i32,doNewline:bool) {
        if self.printPhysical {
            println!("  write  [disk {}, offset {}]  ",disk,off);
            if doNewline {
                println!("");
            }
        }
        self.disks[disk as usize].enqueue(off);
    }

    //
    // mapping for RAID 0 (striping)
    //

    fn bmap0(&self,bnum:i32) -> (i32,i32){
        let cnum = bnum / self.chunkSize;
        let coff = bnum % self.chunkSize;
        return (cnum % self.numDisks,(cnum / self.numDisks) * self.chunkSize + coff);
    }

    

    fn enqueue0(&mut self,addr:i32,size:i32,isWrite:bool) {
        for b in addr..addr+size {
            let (disk,off) = self.bmap0(b);
            if isWrite {
                self.doSingleWrite(disk, off, true);
            }else {
                self.doSingleRead(disk, off, true);
            }
        }

        if self.timing == false && self.printPhysical {
            println!("");
        }
    }

    //
    // mapping for RAID 1 (mirroring)
    //

    fn bmap1(&self,bnum:i32) -> (i32,i32,i32){
        let cnum = bnum / self.chunkSize;
        let coff = bnum % self.chunkSize;
        let disk = 2 * (cnum % (self.numDisks / 2));
        return (disk, disk + 1, (cnum / (self.numDisks / 2)) * self.chunkSize + coff);
    }

    fn enqueue1(&mut self,addr:i32,size:i32,isWrite:bool) {
        for b in addr..addr+size {
            let (disk1,disk2,off) = self.bmap1(b);
            if isWrite {
                self.doSingleWrite(disk1, off, false);
                self.doSingleWrite(disk2, off, true);
            }else {
                if off % 2 == 0{
                    self.doSingleRead(disk1, off, true);
                }else {
                     self.doSingleRead(disk2, off, true);
                }
               
            }
        }

        if self.timing == false && self.printPhysical {
            println!("");
        } 
    }
    // 
    // mapping for RAID 4 (parity disk)
    // 
    // assumes (for now) that there is just one parity disk
    //

    fn bmap4(&self,bnum:i32) -> (i32,i32){
        let cnum = bnum / self.chunkSize;
        let coff = bnum % self.chunkSize;
        return (cnum % (self.numDisks - 1), (cnum / (self.numDisks - 1)) * self.chunkSize + coff);
    }

    fn pmap4(&self, snum:i32)->i32{
        return self.pdisk;
    }

    // 
    // mapping for RAID 5 (rotated parity)
    //
    
    fn __bmap5(&self,bnum:i32) -> (i32,i32,i32){
        let cnum = bnum / self.chunkSize;
        let coff = bnum % self.chunkSize;
        let ddsk = cnum / (self.numDisks - 1);
        let doff = (ddsk * self.chunkSize) + coff;
        let mut disk = cnum % (self.numDisks - 1);
        let col  = (ddsk % self.numDisks);
        let pdsk = (self.numDisks - 1) - col;

        if self.raid5type == "LA" {
            if disk >= pdsk{
                disk += 1;
            }
        }else if self.raid5type == "LS" {
            disk = (disk - col) % (self.numDisks);
        }else {
            println!("error: no such RAID scheme");
        }
        assert!(disk!=pdsk);
        return (disk,pdsk,doff);
    }

    //  yes this is lame (redundant call to __bmap5 is serious programmer laziness)
    fn bmap5(&self,bnum:i32) -> (i32,i32){
        let (disk,pdisk,off) = self.__bmap5(bnum);
        return (disk,off);
    }

    fn pmap5(self, snum:i32)->i32{
        let (disk, pdisk, off) = self.__bmap5(snum * self.blocksInStripe);
        return self.pdisk;
    }


    // RAID 4/5 helper routine to write out some blocks in a stripe
    fn doPartialWrite(&mut self, stripe:i32, begin:i32, end:i32, bmap:fn(i32) -> (i32,i32), pmap:fn(i32)->i32) {
        let numWrites = end - begin;
        let pdisk     = pmap(stripe);

        if (numWrites + 1)<=(self.blocksInStripe - numWrites) {
            let mut offList:Vec<i32> = Vec::new();
            for voff in begin..end {
                let (disk, off) = bmap(voff); 
                self.doSingleRead(disk, off,false);
                if !offList.contains(&off) {
                    offList.push(off);
                }
                for i in 0..offList.len() {
                    self.doSingleRead(pdisk, offList[i], i == (offList.len() - 1));
                }
            }   
        
        }else{
            let stripeBegin = stripe * self.blocksInStripe;
            let stripeEnd   = stripeBegin + self.blocksInStripe;
            for voff in stripeBegin..begin {
                let (disk, off) = bmap(voff); 
                self.doSingleRead(disk, off, (voff == (begin - 1)) && (end == stripeEnd));
            }
            for voff in end..stripeEnd {
                let (disk, off) = bmap(voff); 
                self.doSingleRead(disk, off, voff == (stripeEnd - 1));
            }
        }

        // WRITES: same for additive or subtractive parity
        let mut offList:Vec<i32> = Vec::new();

        for voff in begin..end {
            let (disk, off) = bmap(voff);
            self.doSingleWrite(disk, off,false);
            if !offList.contains(&off) {
                offList.push(off);
            }
        }

        for i in 0..offList.len() {
            self.doSingleWrite(pdisk, offList[i], i == (offList.len() - 1));
        }
    }
    

    // RAID 4/5 enqueue routine
    fn enqueue45(&mut self,addr:i32,size:i32,isWrite:bool) {

        let bmap:fn(i32) -> (i32,i32);
        let pmap:fn(i32)->i32;
        if self.raidLevel == 4 {
            //bmap = bmap4;                                                                     ?????????? function pointer
            //pmap = pmap4;
        }else if self.raidLevel == 5 {
            //bmap = bmap5;
            //pmap = pmap5;
        }

        if isWrite == false {
            for b in addr..addr+size {
                //let (disk, off) = bmap(b);
                //self.doSingleRead(disk, off,false);
            }
        }else {
            let initStripe     = (addr)            / self.blocksInStripe;
            let finalStripe    = (addr + size - 1) / self.blocksInStripe;

            let mut left  = size;
            let mut begin = addr;
            let mut end;

            for stripe in initStripe..finalStripe {
                let endOfStripe = (stripe * self.blocksInStripe) + self.blocksInStripe;

                if left >= self.blocksInStripe {
                    end = begin + self.blocksInStripe;
                } else {
                    end = begin + left;
                }

                if end >= endOfStripe {
                    end = endOfStripe
                }

                //self.doPartialWrite(stripe, begin, end, bmap, pmap);

                left -= (end - begin);
                begin = end;
            }
        }


        if self.timing == false && self.printPhysical {
            println!("");
        } 
        
    }

}






struct RaidOption {
    seed:i32,
    numDisks:i32,
    chunkSize:String,
    numRequests:i32,
    size:String,
    workload:String,
    writeFrac:i32,
    range:i32,
    level:i32,
    raid5type:String,
    reverse:bool,
    timing:bool,
    solve:bool,
}

impl RaidOption {
    fn new() -> RaidOption {
        RaidOption {
            seed:0,
            numDisks:4,
            chunkSize:String::from("4k"),
            numRequests:10,
            size:String::from("4k"),
            workload:String::from("rand"),
            writeFrac:0,
            range:10000,
            level:0,
            raid5type:String::from("LS"),
            reverse:false,
            timing:false,
            solve:false,
        }
    }
}

pub fn raid_op_parse(op_vec:Vec<&str>) {
        let mut raid_op = RaidOption::new();
        let mut i =1;
        while i<op_vec.len() {
            match op_vec[i] {
                "-s" =>{raid_op.seed = op_vec[i+1].parse().unwrap();i = i+2;},
                "-D" =>{raid_op.numDisks = op_vec[i+1].parse().unwrap();i = i+2;},
                "-C" =>{raid_op.chunkSize = op_vec[i+1].to_string();i = i+2;},
                "-n" =>{raid_op.numRequests = op_vec[i+1].parse().unwrap();i = i+2;},
                "-S" =>{raid_op.size = op_vec[i+1].to_string();i=i+2;},
                "-W" =>{raid_op.workload = op_vec[i+1].to_string();i=i+2;},
                "-w" =>{raid_op.writeFrac = op_vec[i+1].parse().unwrap();i=i+2;},
                "-R" =>{raid_op.range = op_vec[i+1].parse().unwrap();i = i+2;},
                "-L" =>{raid_op.level = op_vec[i+1].parse().unwrap();i = i+2;},
                "-5" =>{raid_op.raid5type = op_vec[i+1].to_string();i=i+2;},
                "-r" =>{raid_op.reverse = true;i=i+1;},
                "-t" =>{raid_op.timing = true;i=i+1;},
                "-c" =>{raid_op.solve = true;i=i+1;},
                _ => println!("raid_op_parse match wrong!!"),
            }
        }
        execute_raid_op(raid_op);
}

fn execute_raid_op(options:RaidOption) {

    let seed_u8 = options.seed as u8;
    let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

    let mut rng = SmallRng::from_seed(seed);

    let writeFrac = options.writeFrac as f64 / 100.0f64;

    assert!(writeFrac >= 0.0f64 && writeFrac<=1.0f64);

    let mut size = convert(options.size);

    if size % BLOCKSIZE != 0 {
        println!("error: request size ({}) must be a multiple of BLOCKSIZE ({})" ,size, BLOCKSIZE);
    }

    size = size / BLOCKSIZE;
    let mut workloadIsSequential = false;
    if options.workload == "seq" || options.workload == "s" || options.workload == "sequential" {
        workloadIsSequential = true
    }else if options.workload == "rand" || options.workload == "r" || options.workload == "random" {
        workloadIsSequential = false;
    }else {
        println! ("error: workload must be either r/rand/random or s/seq/sequential");
    }

    assert!(options.level == 0 || options.level == 1 || options.level == 4 || options.level == 5);
    if options.level != 0 && options.numDisks < 2 {
        println!("RAID-4 and RAID-5 need more than 1 disk");
    }

    if options.level == 5 && options.raid5type != "LA" && options.raid5type != "LS"{
        println!("Only two types of RAID-5 supported: left-asymmetric (LA) and left-symmetric (LS) ({} is not)" , options.raid5type);
    }

    //let mut r = 

    let mut off = 0;

    for i in 0..options.numRequests {
        let mut blk;
        if workloadIsSequential == true {
            blk = off;
            off += size;
        }else {
            let rand_x:f64 = rng.gen();
            blk = (rand_x * options.range as f64) as i32;
        }
        let rand_y:f64 = rng.gen();
        if rand_y < writeFrac{
            //r.enqueue(blk,size,true);
        }else {
            //r.enqueue(blk, size, false);
        }
    }

    //let t = r.go();

    if options.timing == false{
        println!(" ");
    }

    if options.solve{
        println!("");
        //r.stats(t)
        println!("");
        //println!("STAT totalTime", t);
        println!("");
    }else{
        println!("");
        println!("Estimate how long the workload should take to complete.");
        println!("- Roughly how many requests should each disk receive?");
        println!("- How many requests are random, how many sequential?");
        println!("");
    }
    
}