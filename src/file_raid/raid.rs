// done
use rand::{Rng, SeedableRng};

const HELP: &str = include_str!("help.txt");

static BLOCKSIZE: i32 = 4096;

fn convert(size: String) -> i32 {
    let length = size.len();
    let lastchar = &size[length - 1..length];
    let mut _nsize = 0;
    if lastchar == "k" || lastchar == "K" {
        let m = 1024;
        _nsize = &size[0..length - 1].parse().unwrap() * m;
    } else if lastchar == "m" || lastchar == "M" {
        let m = 1024 * 1024;
        _nsize = &size[0..length - 1].parse().unwrap() * m;
    } else if lastchar == "g" || lastchar == "G" {
        let m = 1024 * 1024 * 1024;
        _nsize = &size[0..length - 1].parse().unwrap() * m;
    } else {
        _nsize = size.parse().unwrap();
    }
    _nsize
}

struct Disk {
    seek_time: i32,
    xfer_time: f64,
    // length of scheduling queue
    queue_len: i32,
    curr_addr: i32,
    //queue:Vec<>,

    // disk geometry
    num_tracks: i32,
    blocks_per_track: i32,
    blocks_per_disk: i32,

    // stats
    count_io: i32,
    count_seq: i32,
    count_nseq: i32,
    count_rand: i32,
    util_time: f64,
}

impl Disk {
    fn new() -> Disk {
        Disk {
            seek_time: 10,
            xfer_time: 0.1,
            // length of scheduling queue
            queue_len: 8,
            curr_addr: -10000,
            //queue:Vec<>,

            // disk geometry
            num_tracks: 100,
            blocks_per_track: 100,
            blocks_per_disk: 10000,

            // stats
            count_io: 0,
            count_seq: 0,
            count_nseq: 0,
            count_rand: 0,
            util_time: 0.0,
        }
    }
    fn new_a(seek_time: i32, xfer_time: f64, queue_len: i32) -> Disk {
        Disk {
            seek_time: seek_time,
            xfer_time: xfer_time,
            // length of scheduling queue
            queue_len: queue_len,
            curr_addr: -10000,
            //queue:Vec<>,

            // disk geometry
            num_tracks: 100,
            blocks_per_track: 100,
            blocks_per_disk: 10000,

            // stats
            count_io: 0,
            count_seq: 0,
            count_nseq: 0,
            count_rand: 0,
            util_time: 0.0,
        }
    }

    fn stats(&self) -> (i32, i32, i32, i32, f64) {
        return (
            self.count_io,
            self.count_seq,
            self.count_nseq,
            self.count_rand,
            self.util_time,
        );
    }

    fn enqueue(&mut self, addr: i32) {
        assert!(addr < self.blocks_per_disk);

        self.count_io += 1;

        let curr_track = self.curr_addr / self.num_tracks;

        let new_track = addr / self.num_tracks;

        let diff = addr - self.curr_addr;

        if curr_track == new_track || diff < self.blocks_per_track {
            if diff == 1 {
                self.count_seq += 1;
            } else {
                self.count_nseq += 1;
            }
            self.util_time += diff as f64 * self.xfer_time;
        } else {
            self.count_rand += 1;
            self.util_time += self.seek_time as f64 + self.xfer_time;
        }

        self.curr_addr = addr;
    }

    fn go(&self) -> f64 {
        return self.util_time;
    }
}

struct Raid {
    chunk_size: i32,
    num_disks: i32,
    raid_level: i32,
    timing: bool,
    reverse: bool,
    solve: bool,
    raid5type: String,
    disks: Vec<Disk>,
    blocks_in_stripe: i32,
    pdisk: i32,
    print_physical: bool,
}

impl Raid {
    fn new(
        chunk_size: i32,
        num_disks: i32,
        level: i32,
        timing: bool,
        reverse: bool,
        solve: bool,
        raid5type: String,
    ) -> Raid {
        Raid {
            chunk_size: chunk_size,
            num_disks: num_disks,
            raid_level: level,
            timing: timing,
            reverse: reverse,
            solve: solve,
            raid5type: raid5type,
            disks: Vec::new(),
            blocks_in_stripe: 0,
            pdisk: 0,
            print_physical: false,
        }
    }

    fn init(&mut self) {
        if (self.chunk_size & BLOCKSIZE) != 0 {
            println!(
                "chunk_size ({}) must be multiple of blocksize ({}): {}",
                self.chunk_size,
                BLOCKSIZE,
                self.chunk_size % BLOCKSIZE
            );
        }

        if self.raid_level == 1 && self.num_disks & 2 != 0 {
            println!(
                "raid1: disks ({}) must be a multiple of two",
                self.num_disks
            );
        }

        if self.raid_level == 4 {
            self.blocks_in_stripe = (self.num_disks - 1) * self.chunk_size;
            self.pdisk = self.num_disks - 1;
        }

        if self.raid_level == 5 {
            self.blocks_in_stripe = (self.num_disks - 1) * self.chunk_size;
            self.pdisk = -1;
        }

        for _i in 0..self.num_disks {
            self.disks.push(Disk::new());
        }
    }

    fn stats(&self, total_time: f64) {
        for d in 0..self.num_disks {
            let s = self.disks[d as usize].stats();
            if s.4 == total_time {
                println!(
                    "disk:{}  busy: {}  I/Os: {} (sequential:{} nearly:{} random:{})",
                    d,
                    (100.0f64 * s.4 / total_time),
                    s.0,
                    s.1,
                    s.2,
                    s.3
                );
            } else if s.4 == 0.0 {
                println!(
                    "disk:{}  busy: {}  I/Os: {} (sequential:{} nearly:{} random:{})",
                    d,
                    (100.0f64 * s.4 / total_time),
                    s.0,
                    s.1,
                    s.2,
                    s.3
                );
            } else {
                println!(
                    "disk:{}  busy: {}  I/Os: {} (sequential:{} nearly:{} random:{})",
                    d,
                    (100.0f64 * s.4 / total_time),
                    s.0,
                    s.1,
                    s.2,
                    s.3
                );
            }
        }
    }

    fn enqueue(&mut self, addr: i32, size: i32, is_write: bool) {
        if self.timing == false {
            if self.solve || self.reverse == false {
                if is_write {
                    println!("LOGICAL WRITE to  addr:{} size:{}", addr, size * BLOCKSIZE);
                } else {
                    println!("LOGICAL READ to  addr:{} size:{}", addr, size * BLOCKSIZE);
                }

                if self.solve {
                    println!("  Physical reads/writes?");
                }
            } else {
                println!("LOGICAL OPERATION is ?");
            }
        }

        if self.timing == false && (self.solve || self.reverse == true) {
            self.print_physical = true;
        } else {
            self.print_physical = false;
        }

        if self.raid_level == 0 {
            self.enqueue0(addr, size, is_write);
        } else if self.raid_level == 1 {
            self.enqueue1(addr, size, is_write);
        } else if self.raid_level == 4 || self.raid_level == 5 {
            self.enqueue45(addr, size, is_write);
        }
    }

    fn go(&self) -> f64 {
        let mut tmax = 0.0;
        for d in 0..self.num_disks {
            let t = self.disks[d as usize].go();
            if t > tmax {
                tmax = t;
            }
        }
        return tmax;
    }

    // helper functions

    fn do_single_read(&mut self, disk: i32, off: i32, do_newline: bool) {
        if self.print_physical {
            println!("  read  [disk {}, offset {}]  ", disk, off);
            if do_newline {
                println!("");
            }
        }
        self.disks[disk as usize].enqueue(off);
    }

    fn do_single_write(&mut self, disk: i32, off: i32, do_newline: bool) {
        if self.print_physical {
            println!("  write  [disk {}, offset {}]  ", disk, off);
            if do_newline {
                println!("");
            }
        }
        self.disks[disk as usize].enqueue(off);
    }

    //
    // mapping for RAID 0 (striping)
    //

    fn bmap0(&self, bnum: i32) -> (i32, i32) {
        let cnum = bnum / self.chunk_size;
        let coff = bnum % self.chunk_size;
        return (
            cnum % self.num_disks,
            (cnum / self.num_disks) * self.chunk_size + coff,
        );
    }

    fn enqueue0(&mut self, addr: i32, size: i32, is_write: bool) {
        for b in addr..addr + size {
            let (disk, off) = self.bmap0(b);
            if is_write {
                self.do_single_write(disk, off, true);
            } else {
                self.do_single_read(disk, off, true);
            }
        }

        if self.timing == false && self.print_physical {
            println!("");
        }
    }

    //
    // mapping for RAID 1 (mirroring)
    //

    fn bmap1(&self, bnum: i32) -> (i32, i32, i32) {
        let cnum = bnum / self.chunk_size;
        let coff = bnum % self.chunk_size;
        let disk = 2 * (cnum % (self.num_disks / 2));
        return (
            disk,
            disk + 1,
            (cnum / (self.num_disks / 2)) * self.chunk_size + coff,
        );
    }

    fn enqueue1(&mut self, addr: i32, size: i32, is_write: bool) {
        for b in addr..addr + size {
            let (disk1, disk2, off) = self.bmap1(b);
            if is_write {
                self.do_single_write(disk1, off, false);
                self.do_single_write(disk2, off, true);
            } else {
                if off % 2 == 0 {
                    self.do_single_read(disk1, off, true);
                } else {
                    self.do_single_read(disk2, off, true);
                }
            }
        }

        if self.timing == false && self.print_physical {
            println!("");
        }
    }
    //
    // mapping for RAID 4 (parity disk)
    //
    // assumes (for now) that there is just one parity disk
    //

    fn bmap4(&self, bnum: i32) -> (i32, i32) {
        let cnum = bnum / self.chunk_size;
        let coff = bnum % self.chunk_size;
        return (
            cnum % (self.num_disks - 1),
            (cnum / (self.num_disks - 1)) * self.chunk_size + coff,
        );
    }

    fn pmap4(&self, _snum: i32) -> i32 {
        return self.pdisk;
    }

    //
    // mapping for RAID 5 (rotated parity)
    //

    fn __bmap5(&self, bnum: i32) -> (i32, i32, i32) {
        let cnum = bnum / self.chunk_size;
        let coff = bnum % self.chunk_size;
        let ddsk = cnum / (self.num_disks - 1);
        let doff = (ddsk * self.chunk_size) + coff;
        let mut disk = cnum % (self.num_disks - 1);
        let col = ddsk % self.num_disks;
        let pdsk = (self.num_disks - 1) - col;

        if self.raid5type == "LA" {
            if disk >= pdsk {
                disk += 1;
            }
        } else if self.raid5type == "LS" {
            disk = (disk - col) % (self.num_disks);
        } else {
            println!("error: no such RAID scheme");
        }
        assert!(disk != pdsk);
        return (disk, pdsk, doff);
    }

    //  yes this is lame (redundant call to __bmap5 is serious programmer laziness)
    fn bmap5(&self, bnum: i32) -> (i32, i32) {
        let (disk, _pdisk, off) = self.__bmap5(bnum);
        return (disk, off);
    }

    fn pmap5(&self, snum: i32) -> i32 {
        let (_disk, _pdisk, _off) = self.__bmap5(snum * self.blocks_in_stripe);
        return self.pdisk;
    }

    // RAID 4/5 helper routine to write out some blocks in a stripe
    // fn doPartialWrite(&mut self, stripe:i32, begin:i32, end:i32, bmap:fn(i32) -> (i32,i32), pmap:fn(i32)->i32) {
    //     let numWrites = end - begin;
    //     let pdisk     = pmap(stripe);

    //     if (numWrites + 1)<=(self.blocks_in_stripe - numWrites) {
    //         let mut offList:Vec<i32> = Vec::new();
    //         for voff in begin..end {
    //             let (disk, off) = bmap(voff);
    //             self.doSingleRead(disk, off,false);
    //             if !offList.contains(&off) {
    //                 offList.push(off);
    //             }
    //             for i in 0..offList.len() {
    //                 self.doSingleRead(pdisk, offList[i], i == (offList.len() - 1));
    //             }
    //         }

    //     }else{
    //         let stripeBegin = stripe * self.blocks_in_stripe;
    //         let stripeEnd   = stripeBegin + self.blocks_in_stripe;
    //         for voff in stripeBegin..begin {
    //             let (disk, off) = bmap(voff);
    //             self.doSingleRead(disk, off, (voff == (begin - 1)) && (end == stripeEnd));
    //         }
    //         for voff in end..stripeEnd {
    //             let (disk, off) = bmap(voff);
    //             self.doSingleRead(disk, off, voff == (stripeEnd - 1));
    //         }
    //     }

    //     // WRITES: same for additive or subtractive parity
    //     let mut offList:Vec<i32> = Vec::new();

    //     for voff in begin..end {
    //         let (disk, off) = bmap(voff);
    //         self.do_single_write(disk, off,false);
    //         if !offList.contains(&off) {
    //             offList.push(off);
    //         }
    //     }

    //     for i in 0..offList.len() {
    //         self.do_single_write(pdisk, offList[i], i == (offList.len() - 1));
    //     }
    // }

    fn do_partial_write4(&mut self, stripe: i32, begin: i32, end: i32) {
        let num_writes = end - begin;
        let pdisk = self.pmap4(stripe);

        if (num_writes + 1) <= (self.blocks_in_stripe - num_writes) {
            let mut off_list: Vec<i32> = Vec::new();
            for voff in begin..end {
                let (disk, off) = self.bmap4(voff);
                self.do_single_read(disk, off, false);
                if !off_list.contains(&off) {
                    off_list.push(off);
                }
                for i in 0..off_list.len() {
                    self.do_single_read(pdisk, off_list[i], i == (off_list.len() - 1));
                }
            }
        } else {
            let stripe_begin = stripe * self.blocks_in_stripe;
            let stripe_end = stripe_begin + self.blocks_in_stripe;
            for voff in stripe_begin..begin {
                let (disk, off) = self.bmap4(voff);
                self.do_single_read(disk, off, (voff == (begin - 1)) && (end == stripe_end));
            }
            for voff in end..stripe_end {
                let (disk, off) = self.bmap4(voff);
                self.do_single_read(disk, off, voff == (stripe_end - 1));
            }
        }

        // WRITES: same for additive or subtractive parity
        let mut off_list: Vec<i32> = Vec::new();

        for voff in begin..end {
            let (disk, off) = self.bmap4(voff);
            self.do_single_write(disk, off, false);
            if !off_list.contains(&off) {
                off_list.push(off);
            }
        }

        for i in 0..off_list.len() {
            self.do_single_write(pdisk, off_list[i], i == (off_list.len() - 1));
        }
    }

    fn do_partial_write5(&mut self, stripe: i32, begin: i32, end: i32) {
        let num_writes = end - begin;
        let pdisk = self.pmap5(stripe);

        if (num_writes + 1) <= (self.blocks_in_stripe - num_writes) {
            let mut off_list: Vec<i32> = Vec::new();
            for voff in begin..end {
                let (disk, off) = self.bmap5(voff);
                self.do_single_read(disk, off, false);
                if !off_list.contains(&off) {
                    off_list.push(off);
                }
                for i in 0..off_list.len() {
                    self.do_single_read(pdisk, off_list[i], i == (off_list.len() - 1));
                }
            }
        } else {
            let stripe_begin = stripe * self.blocks_in_stripe;
            let stripe_end = stripe_begin + self.blocks_in_stripe;
            for voff in stripe_begin..begin {
                let (disk, off) = self.bmap5(voff);
                self.do_single_read(disk, off, (voff == (begin - 1)) && (end == stripe_end));
            }
            for voff in end..stripe_end {
                let (disk, off) = self.bmap5(voff);
                self.do_single_read(disk, off, voff == (stripe_end - 1));
            }
        }

        // WRITES: same for additive or subtractive parity
        let mut off_list: Vec<i32> = Vec::new();

        for voff in begin..end {
            let (disk, off) = self.bmap5(voff);
            self.do_single_write(disk, off, false);
            if !off_list.contains(&off) {
                off_list.push(off);
            }
        }

        for i in 0..off_list.len() {
            self.do_single_write(pdisk, off_list[i], i == (off_list.len() - 1));
        }
    }

    // RAID 4/5 enqueue routine
    fn enqueue45(&mut self, addr: i32, size: i32, is_write: bool) {
        //let bmap:fn(i32) -> (i32,i32);
        //let pmap:fn(i32)->i32;
        //if self.raid_level == 4 {
        //bmap = bmap4;                                                                     //?????????? function pointer
        //pmap = pmap4;
        //}else if self.raid_level == 5 {
        //bmap = bmap5;
        //pmap = pmap5;
        //}

        // if isWrite == false {
        //     for b in addr..addr+size {
        //         //let (disk, off) = bmap(&self,b);
        //         //self.doSingleRead(disk, off,false);
        //     }
        // }else {
        //     let initStripe     = (addr)            / self.blocks_in_stripe;
        //     let finalStripe    = (addr + size - 1) / self.blocks_in_stripe;

        //     let mut left  = size;
        //     let mut begin = addr;
        //     let mut end;

        //     for stripe in initStripe..finalStripe {
        //         let endOfStripe = (stripe * self.blocks_in_stripe) + self.blocks_in_stripe;

        //         if left >= self.blocks_in_stripe {
        //             end = begin + self.blocks_in_stripe;
        //         } else {
        //             end = begin + left;
        //         }

        //         if end >= endOfStripe {
        //             end = endOfStripe
        //         }

        //         //self.doPartialWrite(stripe, begin, end, bmap, pmap);

        //         left -= (end - begin);
        //         begin = end;
        //     }
        // }

        if self.raid_level == 4 {
            if is_write == false {
                for b in addr..addr + size {
                    let (disk, off) = self.bmap4(b);
                    self.do_single_read(disk, off, false);
                }
            } else {
                let init_stripe = (addr) / self.blocks_in_stripe;
                let final_stripe = (addr + size - 1) / self.blocks_in_stripe;

                let mut left = size;
                let mut begin = addr;
                let mut end;

                for stripe in init_stripe..final_stripe {
                    let end_of_stripe = (stripe * self.blocks_in_stripe) + self.blocks_in_stripe;

                    if left >= self.blocks_in_stripe {
                        end = begin + self.blocks_in_stripe;
                    } else {
                        end = begin + left;
                    }

                    if end >= end_of_stripe {
                        end = end_of_stripe
                    }

                    self.do_partial_write4(stripe, begin, end);

                    left -= end - begin;
                    begin = end;
                }
            }
        } else if self.raid_level == 5 {
            if is_write == false {
                for b in addr..addr + size {
                    let (disk, off) = self.bmap5(b);
                    self.do_single_read(disk, off, false);
                }
            } else {
                let init_stripe = (addr) / self.blocks_in_stripe;
                let final_stripe = (addr + size - 1) / self.blocks_in_stripe;

                let mut left = size;
                let mut begin = addr;
                let mut end;

                for stripe in init_stripe..final_stripe {
                    let end_of_stripe = (stripe * self.blocks_in_stripe) + self.blocks_in_stripe;

                    if left >= self.blocks_in_stripe {
                        end = begin + self.blocks_in_stripe;
                    } else {
                        end = begin + left;
                    }

                    if end >= end_of_stripe {
                        end = end_of_stripe
                    }

                    self.do_partial_write5(stripe, begin, end);

                    left -= end - begin;
                    begin = end;
                }
            }
        }

        if self.timing == false && self.print_physical {
            println!("");
        }
    }
}

struct RaidOption {
    seed: u64,
    num_disks: i32,
    chunk_size: String,
    num_requests: i32,
    size: String,
    workload: String,
    write_frac: i32,
    range: i32,
    level: i32,
    raid5type: String,
    reverse: bool,
    timing: bool,
    solve: bool,
}

impl RaidOption {
    fn new() -> RaidOption {
        RaidOption {
            seed: 0,
            num_disks: 4,
            chunk_size: String::from("4k"),
            num_requests: 10,
            size: String::from("4k"),
            workload: String::from("rand"),
            write_frac: 0,
            range: 10000,
            level: 0,
            raid5type: String::from("LS"),
            reverse: false,
            timing: false,
            solve: false,
        }
    }
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut raid_op = RaidOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" => {
                raid_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-D" => {
                raid_op.num_disks = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-C" => {
                raid_op.chunk_size = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-n" => {
                raid_op.num_requests = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-S" => {
                raid_op.size = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-W" => {
                raid_op.workload = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-w" => {
                raid_op.write_frac = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-R" => {
                raid_op.range = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-L" => {
                raid_op.level = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-5" => {
                raid_op.raid5type = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-r" => {
                raid_op.reverse = true;
                i = i + 1;
            }
            "-t" => {
                raid_op.timing = true;
                i = i + 1;
            }
            "-c" => {
                raid_op.solve = true;
                i = i + 1;
            }
            _ => {
                println!("raid_op_parse match wrong!!");
                return;
            }
        }
    }
    execute_raid_op(raid_op);
}

fn execute_raid_op(options: RaidOption) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(options.seed);

    println!("ARG blockSize {}", BLOCKSIZE);
    println!("ARG seed {}", options.seed);
    println!("ARG num_disks {}", options.num_disks);
    println!("ARG chunk_size {}", options.chunk_size);
    println!("ARG numRequests {}", options.num_requests);
    println!("ARG reqSize {}", options.size);
    println!("ARG workload {}", options.workload);
    println!("ARG writeFrac {}", options.write_frac);
    println!("ARG randRange {}", options.range);
    println!("ARG level {}", options.level);
    println!("ARG raid5 {}", options.raid5type);
    println!("ARG reverse {}", options.reverse);
    println!("ARG timing {}", options.timing);
    println!("");

    let write_frac = options.write_frac as f64 / 100.0f64;

    assert!(write_frac >= 0.0f64 && write_frac <= 1.0f64);

    let mut size = convert(options.size);

    if size % BLOCKSIZE != 0 {
        println!(
            "error: request size ({}) must be a multiple of BLOCKSIZE ({})",
            size, BLOCKSIZE
        );
    }

    size = size / BLOCKSIZE;
    let mut workload_is_sequential = false;
    if options.workload == "seq" || options.workload == "s" || options.workload == "sequential" {
        workload_is_sequential = true
    } else if options.workload == "rand" || options.workload == "r" || options.workload == "random"
    {
        workload_is_sequential = false;
    } else {
        println!("error: workload must be either r/rand/random or s/seq/sequential");
    }

    assert!(options.level == 0 || options.level == 1 || options.level == 4 || options.level == 5);
    if options.level != 0 && options.num_disks < 2 {
        println!("RAID-4 and RAID-5 need more than 1 disk");
    }

    if options.level == 5 && options.raid5type != "LA" && options.raid5type != "LS" {
        println!("Only two types of RAID-5 supported: left-asymmetric (LA) and left-symmetric (LS) ({} is not)" , options.raid5type);
    }

    let mut r = Raid::new(
        convert(options.chunk_size),
        options.num_disks,
        options.level,
        options.timing,
        options.reverse,
        options.solve,
        options.raid5type,
    );
    r.init();

    let mut off = 0;

    for _i in 0..options.num_requests {
        let blk;
        if workload_is_sequential == true {
            blk = off;
            off += size;
        } else {
            let rand_x: f64 = rng.gen();
            blk = (rand_x * options.range as f64) as i32;
        }
        let rand_y: f64 = rng.gen();
        if rand_y < write_frac {
            r.enqueue(blk, size, true);
        } else {
            r.enqueue(blk, size, false);
        }
    }

    let t = r.go();

    if options.timing == false {
        println!(" ");
    }

    if options.solve {
        println!("");
        r.stats(t);
        println!("");
        println!("STAT totalTime {}", t);
        println!("");
    } else {
        println!("");
        println!("Estimate how long the workload should take to complete.");
        println!("- Roughly how many requests should each disk receive?");
        println!("- How many requests are random, how many sequential?");
        println!("");
    }
}
