//done
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

const HELP: &str = include_str!("help.txt");

static mut DEBUG: bool = false;
static mut printOps: bool = true;
static mut printState: bool = true;
static mut printFinal: bool = true;

fn dprint(s: &str) {
    unsafe {
        if DEBUG {
            println!("{}", s);
        }
    }
}
#[derive(Debug)]
struct Bitmap {
    size: usize,
    bmap: Vec<i32>,
}

impl Bitmap {
    fn new(size: usize) -> Bitmap {
        Bitmap {
            size: size,
            bmap: vec![0; size],
        }
    }

    fn alloc(&mut self) -> i32 {
        for num in 0..self.bmap.len() {
            if self.bmap[num] == 0 {
                self.bmap[num] = 1;
                return num as i32;
            }
        }
        return -1;
    }

    fn free(&mut self, num: usize) {
        assert_eq!(self.bmap[num], 1);
        self.bmap[num] = 0;
    }

    fn mark_allocated(&mut self, num: usize) {
        assert_eq!(self.bmap[num], 0);
        self.bmap[num] = 1;
    }

    fn dump(&mut self) -> String {
        let mut s = "".to_string();
        for i in 0..self.bmap.len() {
            s += &self.bmap[i].to_string();
        }
        s
    }
}
#[derive(Debug)]
struct Block {
    ftype: String,
    dirUsed: i32,
    maxUsed: i32,
    dirList: Vec<(String, i32)>,
    data: String,
}

impl Block {
    fn new(ftype: String) -> Block {
        //if (ftype == "d".to_string()) || (ftype == "f".to_string()) || (ftype == "free".to_string()) {
        Block {
            ftype: ftype,
            dirUsed: 0,
            maxUsed: 32,
            dirList: Vec::new(),
            data: String::from(""),
        }
        //}
    }

    fn dump(&mut self) -> String {
        if self.ftype == "free".to_string() {
            return String::from("[]");
        } else if self.ftype == "d".to_string() {
            let mut rc = String::from("");
            for d in &self.dirList {
                let short = format!("({},{})", d.0, d.1);
                if rc == "" {
                    rc = short;
                } else {
                    rc += &(" ".to_string() + &short);
                }
            }
            return format!("[{}]", rc);
        } else {
            return format!("[{}]", self.data);
        }
    }

    fn set_type(&mut self, ftype: String) {
        assert_eq!(self.ftype, "free");
        self.ftype = ftype;
    }

    fn add_data(&mut self, data: String) {
        assert_eq!(self.ftype, "f");
        self.data = data;
    }

    fn get_num_entries(&mut self) -> i32 {
        assert_eq!(self.ftype, "d");
        self.dirUsed
    }

    fn get_free_entries(&mut self) -> i32 {
        assert_eq!(self.ftype, "d");
        self.maxUsed - self.dirUsed
    }

    fn get_entry(&mut self, num: usize) -> (String, i32) {
        assert_eq!(self.ftype, "d");
        if num >= self.dirUsed as usize {
            println!("Error exit()");
        }
        let na = &self.dirList[num].0;
        let n = &self.dirList[num].1;
        (na.to_string(), *n)
    }

    fn add_dir_entry(&mut self, name: String, inum: i32) {
        assert_eq!(self.ftype, "d");
        self.dirList.push((name, inum));
        self.dirUsed += 1;
    }

    fn del_dir_entry(&mut self, name: &String) {
        assert_eq!(self.ftype, "d");
        let tname: Vec<&str> = name.split('/').collect();

        let dname = tname[(tname.len() - 1) as usize];

        for i in 0..self.dirList.len() {
            if self.dirList[i].0 == dname {
                self.dirList.remove(i);
                self.dirUsed -= 1;
                break;
            }
        }
        //assert(1 == 0)
    }

    fn dir_entry_exists(&mut self, name: String) -> bool {
        assert_eq!(self.ftype, "d");
        for d in &self.dirList {
            if name == d.0 {
                return true;
            }
        }
        return false;
    }

    fn free(&mut self) {
        assert!(self.ftype != "d", "self.ftype != d");
        if self.ftype == "d" {
            assert_eq!(self.dirUsed, 2);
            self.dirUsed = 0;
        }
        self.data = "".to_string();
        self.ftype = "free".to_string();
    }
}
#[derive(Debug)]
struct Inode {
    ftype: String,
    addr: i32,
    refCnt: i32,
}

impl Inode {
    fn new() -> Inode {
        Inode {
            ftype: String::from("free"),
            addr: -1,
            refCnt: 1,
        }
    }

    fn new_a(ftype: String, addr: i32, refCnt: i32) -> Inode {
        assert!(&*ftype == "d" || &*ftype == "f" || &*ftype == "free");
        Inode {
            ftype: ftype,
            addr: addr,
            refCnt: refCnt,
        }
    }

    fn set_all(&mut self, ftype: String, addr: i32, refCnt: i32) {
        self.ftype = ftype;
        self.addr = addr;
        self.refCnt = refCnt;
    }

    fn inc_ref_cnt(&mut self) {
        self.refCnt += 1;
    }

    fn dec_ref_cnt(&mut self) {
        self.refCnt -= 1;
    }

    fn get_ref_cnt(&mut self) -> i32 {
        return self.refCnt;
    }

    fn set_type(&mut self, ftype: String) {
        assert!(&*ftype == "d" || &*ftype == "f" || &*ftype == "free");
        self.ftype = ftype;
    }

    fn set_addr(&mut self, block: i32) {
        self.addr = block;
    }

    fn get_size(&mut self) -> i32 {
        if self.addr == -1 {
            return 0;
        } else {
            return 1;
        }
    }

    fn get_addr(&self) -> i32 {
        return self.addr;
    }

    fn get_type(&self) -> String {
        let x = &self.ftype;
        return x.to_string();
    }

    fn free(&mut self) {
        self.ftype = "free".to_string();
        self.addr = -1;
    }
}
#[derive(Debug)]
struct Fs {
    numInodes: i32,
    numData: i32,

    ibitmap: Bitmap,
    inodes: Vec<Inode>,

    dbitmap: Bitmap,
    data: Vec<Block>,

    ROOT: usize,

    files: Vec<String>,
    dirs: Vec<String>,
    nameToInum: HashMap<String, i32>,
}

impl Fs {
    fn new(numInodes: i32, numData: i32) -> Fs {
        Fs {
            numInodes: numInodes,
            numData: numData,

            ibitmap: Bitmap::new(numInodes as usize),
            //inodes:vec![Inode::new();numInodes as usize],
            inodes: Vec::new(),

            dbitmap: Bitmap::new(numData as usize),
            //data:vec![Block::new("free".into());numData as usize],
            data: Vec::new(),

            ROOT: 0,

            files: Vec::new(),
            dirs: vec!["/".into()],
            nameToInum: HashMap::new(),
        }
    }

    fn create_root_directory(&mut self) {
        for i in 0..self.numInodes {
            self.inodes.push(Inode::new());
        }

        for i in 0..self.numData {
            self.data.push(Block::new("free".into()));
        }

        self.ibitmap.mark_allocated(self.ROOT);
        self.inodes[self.ROOT].set_all("d".into(), 0, 2);
        self.dbitmap.mark_allocated(self.ROOT);
        self.data[0].set_type("d".into());
        self.data[0].add_dir_entry(".".into(), self.ROOT as i32);
        self.data[0].add_dir_entry("..".into(), self.ROOT as i32);

        self.nameToInum.insert("/".into(), self.ROOT as i32);
    }

    fn dump(&mut self) {
        println!("inode bitmap {}", self.ibitmap.dump());
        print!("inodes       ");
        for i in 0..self.numInodes {
            let ftype = self.inodes[i as usize].get_type();
            if ftype == "free" {
                print!("[]");
            } else {
                print!(
                    "[{} a:{} r:{}]",
                    ftype,
                    self.inodes[i as usize].get_addr(),
                    self.inodes[i as usize].get_ref_cnt()
                );
            }
        }
        println!("");
        println!("data bitmap   {}", self.dbitmap.dump());
        print!("data        ");
        for i in 0..self.numData {
            print!("{} ", self.data[i as usize].dump());
        }
        println!("");
    }

    fn make_name(&mut self) -> String {
        // let seed_u8 = 0;
        // let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
        //                             seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
        //                             seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
        //                             seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

        // let mut rng = SmallRng::from_seed(seed);

        let p = [
            "a", "b", "c", "d", "e", "f", "g", "h", "j", "k", "m", "n", "o", "p", "q", "r", "s",
            "t", "u", "v", "w", "x", "y", "z",
        ];
        let rand_x: f32 = rand::thread_rng().gen();
        return p[(rand_x * p.len() as f32) as usize].to_string();
        // p = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 's', 't', 'v', 'w', 'x', 'y', 'z']
        // f = p[int(random.random() * len(p))]
        // p = ['a', 'e', 'i', 'o', 'u']
        // s = p[int(random.random() * len(p))]
        // p = ['b', 'c', 'd', 'f', 'g', 'j', 'k', 'l', 'm', 'n', 'p', 's', 't', 'v', 'w', 'x', 'y', 'z']
        // l = p[int(random.random() * len(p))]
        // return '%c%c%c' % (f, s, l)

        //  ??? why two return place there ?
    }

    fn inode_alloc(&mut self) -> i32 {
        return self.ibitmap.alloc();
    }

    fn inode_free(&mut self, inum: usize) {
        self.ibitmap.free(inum);
        self.inodes[inum].free();
    }

    fn dataAlloc(&mut self) -> i32 {
        return self.dbitmap.alloc();
    }

    fn data_free(&mut self, bnum: usize) {
        self.dbitmap.free(bnum);
        self.data[bnum].free();
    }

    fn get_parent(&mut self, name: &String) -> String {
        let tmp: Vec<&str> = name.split("/").collect();
        if tmp.len() == 2 {
            return "/".to_string();
        }

        let mut pname = String::from("");

        for i in 1..(tmp.len() - 1) as usize {
            pname = format!(" {} / {} ", pname, tmp[i]);
        }

        return pname;
    }

    fn delete_flie(&mut self, tfile: String) -> i32 {
        unsafe {
            if printOps {
                println!("unlink(\"{}\")", tfile);
            }
        }
        let inum = self.nameToInum[&tfile] as usize;
        if self.inodes[inum].get_ref_cnt() == 1 {
            let dblock = self.inodes[inum].get_addr();
            //if dblock != -1 {
            self.data_free(dblock as usize);
            //}
            self.inode_free(inum);
        } else {
            self.inodes[inum].dec_ref_cnt();
        }

        let parent = self.get_parent(&tfile);

        let pinum = self.nameToInum[&parent];

        let pblock = self.inodes[pinum as usize].get_addr();

        self.inodes[pinum as usize].dec_ref_cnt();

        self.data[pblock as usize].del_dir_entry(&tfile);

        //self.files.remove(tfile);
        for i in 0..self.files.len() {
            if self.files[i] == tfile {
                self.files.remove(i);
                break;
            }
        }

        return 0;
    }

    fn create_link(&mut self, target: &String, newfile: &String, parent: &String) -> i32 {
        let parentInum = self.nameToInum[parent];

        let pblock = self.inodes[parentInum as usize].get_addr();

        if self.data[pblock as usize].get_free_entries() <= 0 {
            dprint("*** createLink failed: no room in parent directory ***");
            return -1;
        }

        if self.data[pblock as usize].dir_entry_exists(newfile.to_string()) {
            dprint("*** createLink failed: not a unique name ***");
            return -1;
        }

        let tinum = self.nameToInum[target];
        self.inodes[tinum as usize].inc_ref_cnt();

        self.inodes[parentInum as usize].inc_ref_cnt();

        let tmp: Vec<&str> = newfile.split("/").collect();

        let ename = tmp[(tmp.len() - 1) as usize];

        self.data[pblock as usize].add_dir_entry(ename.to_string(), tinum);
        return tinum;
    }

    fn create_file(&mut self, parent: &String, newfile: &String, ftype: &String) -> i32 {
        let parentInum = self.nameToInum[parent];

        let pblock = self.inodes[parentInum as usize].get_addr();

        if self.data[pblock as usize].get_free_entries() <= 0 {
            dprint("*** createLink failed: no room in parent directory ***");
            return -1;
        }

        if self.data[pblock as usize].dir_entry_exists(newfile.to_string()) {
            dprint("*** createLink failed: not a unique name ***");
            return -1;
        }

        let inum = self.inode_alloc();

        if inum == -1 {
            dprint("*** createFile failed: no inodes left ***");
            return -1;
        }

        let mut fblock = -1;
        let refCnt: i32;
        if ftype == "d" {
            refCnt = 2;
            fblock = self.dataAlloc();
            if fblock == -1 {
                dprint("*** createFile failed: no data blocks left ***");
                self.inode_free(inum as usize);
                return -1;
            } else {
                self.data[fblock as usize].set_type("d".to_string());
                self.data[fblock as usize].add_dir_entry(".".to_string(), inum);
                self.data[fblock as usize].add_dir_entry("..".to_string(), parentInum);
            }
        } else {
            refCnt = -1;
        }

        self.inodes[inum as usize].set_all(ftype.to_string(), fblock, refCnt);

        self.inodes[parentInum as usize].inc_ref_cnt();

        self.data[pblock as usize].add_dir_entry(newfile.to_string(), inum);

        return inum;
    }

    fn write_file(&mut self, tfile: &String, data: String) -> i32 {
        let inum = self.nameToInum[tfile];

        let curSize = self.inodes[inum as usize].get_size();

        //dprint("writeFile: inum:{} cursize:{} refcnt:{}" ,inum, curSize, self.inodes[inum].getRefCnt());

        if curSize == 1 {
            dprint("*** writeFile failed: file is full ***");
            return -1;
        }

        let fblock = self.dataAlloc();

        if fblock == -1 {
            dprint("*** writeFile failed: no data blocks left ***");
            return -1;
        } else {
            self.data[fblock as usize].set_type('f'.to_string());
            self.data[fblock as usize].add_data(data);
        }

        self.inodes[inum as usize].set_addr(fblock);

        unsafe {
            if printOps {
                //println! ("fd=open(\"{}\", O_WRONLY|O_APPEND); write(fd, buf, BLOCKSIZE); close(fd);" , tfile);
            }
        }

        return 0;
    }

    fn do_delete(&mut self) -> i32 {
        // let seed_u8 = 0;
        // let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
        //                             seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
        //                             seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
        //                             seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

        // let mut rng = SmallRng::from_seed(seed);
        dprint("DO Delete");
        if self.files.len() == 0 {
            return -1;
        }
        let rand_x: f32 = rand::thread_rng().gen();
        //let rand_x:f32 = rng.gen();

        let dfile = &self.files[(rand_x * self.files.len() as f32) as usize];

        return self.delete_flie(dfile.to_string());
    }

    fn do_link(&mut self) -> i32 {
        dprint("doLink");
        // let seed_u8 = 0;
        // let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
        //                             seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
        //                             seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
        //                             seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

        // let mut rng = SmallRng::from_seed(seed);
        // let rand_x:f32 = rng.gen();
        let rand_x: f32 = rand::thread_rng().gen();
        if self.files.len() == 0 {
            return -1;
        }
        let parent = self.dirs[(rand_x * self.dirs.len() as f32) as usize].clone();
        let nfile = self.make_name().clone();
        let rand_y: f32 = rand::thread_rng().gen();
        let target = self.files[(rand_y * self.files.len() as f32) as usize].clone();

        let mut fullname = String::from("");
        if parent == "/" {
            fullname = format!("{}{}", parent, nfile);
        } else {
            fullname = format!("{}/{}", parent, nfile);
        }

        let inum = self.create_link(&target, &nfile, &parent);

        if inum >= 0 {
            self.files.push(fullname.clone());
            self.nameToInum.insert(fullname.clone(), inum);

            // if printOps {

            // }
            return 0;
        }
        return -1;
    }

    fn do_create(&mut self, ftype: &String) -> i32 {
        dprint("doCreate");
        // let seed_u8 = 0;
        // let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
        //                             seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
        //                             seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
        //                             seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

        // let mut rng = SmallRng::from_seed(seed);

        // let rand_x:f32 = rng.gen();
        let rand_x: f32 = rand::thread_rng().gen();

        let mut parent = self.dirs[(rand_x * self.dirs.len() as f32) as usize].clone();
        let nfile = self.make_name().clone();

        // let mut tlist:Vec<String> = Vec::new();
        // if ftype == "d" {
        //     tlist = self.dirs.clone();
        // }else{
        //     tlist = self.files.clone();
        // }
        let mut fullname = String::from("");
        if parent == "/" {
            fullname = format!("{}{}", parent, nfile);
        } else {
            fullname = format!("{}/{}", parent, nfile);
        }

        let inum = self.create_file(&parent, &nfile, ftype);
        if inum >= 0 {
            //tlist.push(fullname.clone());
            if ftype == "d" {
                self.dirs.push(fullname.clone());
            } else {
                self.files.push(fullname.clone());
            }
            self.nameToInum.insert(fullname.clone(), inum);

            if parent == "/" {
                parent = "".to_string();
            }

            if ftype == "d" {
                unsafe {
                    if printOps {
                        println!("mkdir(\"{}/{}\");", parent, nfile);
                    }
                }
            } else {
                unsafe {
                    if printOps {
                        println!("creat(\"{}/{}\");", parent, nfile);
                    }
                }
            }

            return 0;
        }

        return -1;
    }

    fn do_append(&mut self) -> i32 {
        dprint("doAppend");
        // let seed_u8 = 0;
        // let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
        //                             seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
        //                             seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
        //                             seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

        // let mut rng = SmallRng::from_seed(seed);

        // let rand_x:f32 = rng.gen();
        let rand_x: f32 = rand::thread_rng().gen();

        if self.files.len() == 0 {
            return -1;
        }

        let afile = self.files[(rand_x * self.files.len() as f32) as usize].clone();
        dprint("try writeFile afile");

        let rand_y: f32 = rand::thread_rng().gen();
        let data = ((97 + (rand_y * 26f32) as u8) as char).to_string();

        let rc = self.write_file(&afile, data);

        return rc;
    }

    fn run(&mut self, numRequests: i32) {
        //println!("it's ok");

        println!("Initial state");
        println!("");
        self.dump();
        println!("");

        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

        for i in 0..numRequests {
            unsafe {
                if printOps == false {
                    println!("Which operation took place?");
                }
            }
            let mut rc = -1;
            while rc == -1 {
                let r: f32 = rng.gen();
                if r < 0.3f32 {
                    rc = self.do_append();
                    println!("do_append rc:{}", rc);
                } else if r < 0.5f32 {
                    rc = self.do_delete();
                    println!("do_delete rc:{}", rc);
                } else if r < 0.7f32 {
                    rc = self.do_link();
                    println!("do_link rc:{}", rc);
                } else {
                    let rand: f32 = rng.gen();
                    if rand < 0.75f32 {
                        rc = self.do_create(&"f".to_string());
                        println!("doCreate(f) rc: {} ", rc)
                    } else {
                        rc = self.do_create(&"d".to_string());
                        println!("doCreate(d) rc: {} ", rc)
                    }
                }
            }

            unsafe {
                if printState == true {
                    println!("");
                    self.dump();
                    println!("");
                } else {
                    println!("");
                    println!("  State of file system (inode bitmap, inodes, data bitmap, data)?");
                    println!("");
                }
            }
        }

        unsafe {
            if printFinal {
                println!("");
                println!("Summary of files, directories::");
                println!("");
                println!("  Files:      {:?}", self.files);
                println!("  Directories:{:?}", self.dirs);
                println!("");
            }
        }
    }
}

struct VsfsOption {
    seed: u64,
    numInodes: i32,
    numData: i32,
    numRequests: i32,
    reverse: bool,
    printFinal: bool,
    solve: bool,
}

impl VsfsOption {
    fn new() -> VsfsOption {
        VsfsOption {
            seed: 0,
            numInodes: 8,
            numData: 8,
            numRequests: 10,
            reverse: false,
            printFinal: false,
            solve: false,
        }
    }
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut vsfs_op = VsfsOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" => {
                vsfs_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-i" => {
                vsfs_op.numInodes = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-d" => {
                vsfs_op.numData = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-n" => {
                vsfs_op.numRequests = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-r" => {
                vsfs_op.reverse = true;
                i = i + 1;
            }
            "-p" => {
                vsfs_op.printFinal = true;
                i = i + 1;
            }
            "-c" => {
                vsfs_op.solve = true;
                i = i + 1;
            }
            _ => {
                println!("vsfs_op_parse match wrong!!");
                return;
            }
        }
    }
    execute_vsfs_op(vsfs_op);
}

fn execute_vsfs_op(options: VsfsOption) {
    // let seed_u8 = options.seed as u8;
    // let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
    //                                 seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
    //                                 seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
    //                                 seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

    // let mut rng = SmallRng::from_seed(seed);

    println!("ARG seed {} ", options.seed);
    println!("ARG numInodes {} ", options.numInodes);
    println!("ARG numData {} ", options.numData);
    println!("ARG numRequests {} ", options.numRequests);
    println!("ARG reverse {} ", options.reverse);
    println!("ARG printFinal {} ", options.printFinal);
    println!("");

    unsafe {
        if options.reverse {
            printState = false;
            printOps = true;
        } else {
            printState = true;
            printOps = false;
        }

        if options.solve {
            printOps = true;
            printState = true;
        }

        printFinal = options.printFinal;
    }

    let mut f = Fs::new(options.numInodes, options.numData);
    f.create_root_directory();

    f.run(options.numRequests);
}
