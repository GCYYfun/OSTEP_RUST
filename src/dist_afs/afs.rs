use rand::{Rng, SeedableRng};
use std::collections::HashMap;

const HELP: &str = include_str!("help.txt");

// fn pickrand<T> (tlist:Vec<T>) -> T{
//     let rand_x:f64 =  rand::thread_rng().gen();
//     let n = (rand_x * tlist.len() as f64) as usize;
//     let p = tlist[n].clone();
//     return p;
// }

fn pickrandS(tlist: Vec<String>) -> String {
    let rand_x: f64 = rand::thread_rng().gen();
    let n = (rand_x * tlist.len() as f64) as usize;
    let p = tlist[n].clone();
    return p;
}

fn pickrandC(tlist: &mut Vec<Client>) -> &mut Client {
    let rand_x: f64 = rand::thread_rng().gen();
    let n = (rand_x * tlist.len() as f64) as usize;
    let p = &mut tlist[n];
    return p;
}

fn isset(num: i32, index: i32) -> bool {
    let mask = 1 << index;
    return (num & mask) > 0;
}

fn dospace(howmuch: i32) {
    for _i in 0..howmuch + 1 {
        print!("{:>width$}", width = 28);
    }
}

//
//  Which files are used in the simulation
//
//  Not representing a realistic piece of anything
//  but rather just for convenience when generating
//  random traces ...
//
//  Files are named 'a', 'b', etc. for ease of use
//  Could probably add a numeric aspect to allow
//  for more than 26 files but who cares
//
#[derive(Clone)]
struct Files {
    numfiles: i32,
    value: i32,
    filelist: Vec<String>,
}

impl Files {
    fn new(numfiles: i32) -> Files {
        Files {
            numfiles: numfiles,
            value: 0,
            filelist: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
                "f".to_string(),
                "g".to_string(),
                "h".to_string(),
                "i".to_string(),
                "j".to_string(),
                "k".to_string(),
                "l".to_string(),
                "m".to_string(),
                "n".to_string(),
                "o".to_string(),
                "p".to_string(),
                "q".to_string(),
                "r".to_string(),
                "s".to_string(),
                "t".to_string(),
                "u".to_string(),
                "v".to_string(),
                "w".to_string(),
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
            ],
        }
    }

    fn init(&mut self) {
        self.filelist = self.filelist[0..self.numfiles as usize].to_vec();
    }

    fn getfiles(&self) -> Vec<String> {
        return self.filelist.clone();
    }

    fn getvalue(&mut self) -> i32 {
        let rc = self.value;
        self.value += 1;
        return rc;
    }
}

#[derive(Clone)]
struct Server {
    files: Files,
    solve: bool,
    detail: i32,
    contents: HashMap<String, i32>,
    getcnt: i32,
    putcnt: i32,
    clients: Vec<Client>,
    cache: HashMap<String, Vec<String>>,
}

impl Server {
    fn new(files: Files, solve: bool, detail: i32) -> Server {
        Server {
            files: files,
            solve: solve,
            detail: detail,
            contents: HashMap::new(),
            getcnt: 0,
            putcnt: 0,
            clients: Vec::new(),
            cache: HashMap::new(),
        }
    }

    fn init(&mut self) {
        let flist = self.files.getfiles();
        for f in flist {
            let v = self.files.getvalue();
            self.contents.insert(f, v);
        }
    }

    fn stats(self) {
        println!("Server   -- Gets:{} Puts:{}", self.getcnt, self.putcnt);
    }

    fn filestats(&self, printcontents: bool) {
        for fname in self.contents.keys() {
            if printcontents {
                println!("file:{} contains:{}", fname, self.contents[fname]);
            } else {
                println!("file:{} contains:?", fname);
            }
        }
    }

    fn setclients(&mut self, clients: Vec<Client>) {
        self.clients = clients;

        for c in &self.clients {
            self.cache.insert(c.getname(), Vec::new());
        }
    }

    fn get(&mut self, client: String, fname: String) -> i32 {
        assert!(
            self.contents.contains_key(&fname),
            "server:get() -- file:{} not found on server",
            fname
        );
        self.getcnt += 1;
        if self.solve && isset(self.detail, 0) {
            println!("getfile:{} c:{} [{}]", fname, client, self.contents[&fname]);
        }
        if !self.cache[&client].contains(&fname) {
            let mut v = self.cache[&client].clone();
            v.push(fname.clone());
            self.cache.insert(client, v);
        }
        return self.contents[&fname];
    }

    fn put(&mut self, client: String, fname: String, value: i32) {
        assert!(
            self.contents.contains_key(&fname),
            "server:put() -- file:{} not found on server",
            fname.clone()
        );
        self.putcnt += 1;
        self.contents.insert(fname.clone(), value);
        if self.solve && isset(self.detail, 0) {
            println!(
                "getfile:{} c:{} [{}]",
                fname.clone(),
                client,
                self.contents[&fname]
            );
        }
        for c in &mut self.clients {
            let cname = c.getname();
            if self.cache[&cname].contains(&fname) && cname != client {
                if self.solve && isset(self.detail, 0) {
                    println!("callback: c:{} file:{}", cname, &fname);
                }
                c.invalidate(fname.clone());
                for i in 0..self.cache[&cname].len() {
                    if self.cache[&cname][i] == fname {
                        let mut v = self.cache[&cname].clone();
                        v.remove(i);
                        self.cache.insert(cname.clone(), v);
                        break;
                    }
                }
                //  eq ↑
                //self.cache[cname].remove(fname)
            }
        }
    }
}

#[derive(Clone)]
struct FileDesc {
    max: i32,
    fd: HashMap<i32, String>,
}

impl FileDesc {
    fn new() -> FileDesc {
        FileDesc {
            max: 1024,
            fd: HashMap::new(),
        }
    }

    fn new_a(max: i32) -> FileDesc {
        FileDesc {
            max: max,
            fd: HashMap::new(),
        }
    }

    fn init(&mut self) {
        for i in 0..self.max {
            self.fd.insert(i, "".to_string());
        }
    }

    fn alloc(&mut self, fname: String, sfd: i32) -> i32 {
        if sfd != -1 {
            assert!(
                self.fd[&sfd] == "",
                "filedesc:alloc() -- fd:{} already in use, cannot allocate",
                sfd
            );
            self.fd.insert(sfd, fname);
            return sfd;
        } else {
            for i in 0..self.max {
                if self.fd[&i] == "" {
                    self.fd.insert(i, fname);
                    return i;
                }
            }
        }
        return -1;
    }

    fn lookup(&self, sfd: i32) -> String {
        //assert!(sfd >= 0 && sfd < self.max, "filedesc:lookup() -- file descriptor out of valid range ({} not between 0 and {})" ,sfd, self.max);
        assert!(
            self.fd[&sfd] != "",
            "filedesc:lookup() -- fd:{} not in use, cannot lookup",
            sfd
        );
        return self.fd[&sfd].clone();
    }

    fn free(&self, i: i32) {
        // assert!(i >= 0 && i < self.max, "filedesc:free() -- file descriptor out of valid range (%d not between 0 and %d)" ,sfd, self.max);
        // assert!(self.fd[&sfd] != "",    "filedesc:free() -- fd:{} not in use, cannot free" , sfd);
        // self.fd.insert(i,"");
    }
}

#[derive(Clone)]
struct Cache {
    name: String,
    num: i32,
    solve: bool,
    detail: i32,

    cache: HashMap<String, CacheContent>,

    hitcnt: i32,
    misscnt: i32,
    invalidcnt: i32,
}

#[derive(Default, Clone, Debug)]
struct CacheContent {
    data: i32,
    dirty: bool,
    refcnt: i32,
    valid: bool,
}

impl Cache {
    fn new(name: String, num: i32, solve: bool, detail: i32) -> Cache {
        Cache {
            name: name,
            num: num,
            solve: solve,
            detail: detail,

            cache: HashMap::new(),

            hitcnt: 0,
            misscnt: 0,
            invalidcnt: 0,
        }
    }

    fn stats(&self) {
        println!(
            "   Cache -- Hits:{} Misses:{} Invalidates:{}",
            self.hitcnt, self.misscnt, self.invalidcnt
        );
    }

    fn put(&mut self, fname: String, data: i32, dirty: bool, refcnt: i32) {
        let cc = CacheContent {
            data: data,
            dirty: dirty,
            refcnt: refcnt,
            valid: true,
        };
        self.cache.insert(fname, cc);
    }

    fn update(&mut self, fname: String, data: i32) {
        let cc = CacheContent {
            data: data,
            dirty: true,
            refcnt: self.cache[&fname].refcnt,
            valid: self.cache[&fname].valid,
        };
        self.cache.insert(fname, cc);
    }

    fn invalidate(&mut self, fname: String) {
        assert!(
            self.cache.contains_key(&fname),
            "cache:invalidate() -- cannot invalidate file not in cache ({})",
            fname
        );
        self.invalidcnt += 1;
        let cc = CacheContent {
            data: self.cache[&fname].data,
            dirty: self.cache[&fname].dirty,
            refcnt: self.cache[&fname].refcnt,
            valid: false,
        };
        self.cache.insert(fname.clone(), cc);
        if self.solve && isset(self.detail, 1) {
            dospace(self.num);
            if isset(self.detail, 3) {
                println!("{} invalidate {}", self.name, fname);
            } else {
                println!("invalidate {}", fname);
            }

            // self.printstate(self.num);  ???
        }
    }

    fn checkvalid(&self, fname: String) {
        assert!(
            self.cache.contains_key(&fname),
            "cache:checkvalid() -- cannot checkvalid on file not in cache ({})",
            fname
        );
        if self.cache[&fname].valid == false && self.cache[&fname].refcnt == 0 {
            // del ?    No Del;
        }
    }

    fn printstate(&self, fname: String) {
        //for fname in self.cache.keys() {            // ??? for ? if ?
        if self.cache.contains_key(&fname) {
            // let data = self.cache[fname].data;
            // let dirty  = self.cache[fname].dirty;
            // let refcnt = self.cache[fname].refcnt;
            // let valid  = self.cache[fname].valid;
            let data = self.cache[&fname].data;
            let dirty = self.cache[&fname].dirty;
            let refcnt = self.cache[&fname].refcnt;
            let valid = self.cache[&fname].valid;
            let mut validPrint: i32;
            let mut dirtyPrint: i32;
            if valid == true {
                validPrint = 1;
            } else {
                validPrint = 0;
            }
            if dirty == true {
                dirtyPrint = 1;
            } else {
                dirtyPrint = 0;
            }

            if self.solve && isset(self.detail, 2) {
                dospace(self.num);
                if isset(self.detail, 3) {
                    println!(
                        "{} [{}:{} (v={},d={},r={})]",
                        self.name, fname, data, validPrint, dirtyPrint, refcnt
                    );
                } else {
                    println!(
                        "[{}:{} (v={},d={},r={})]",
                        fname, data, validPrint, dirtyPrint, refcnt
                    );
                }
            }
        }
    }

    fn checkget(&mut self, fname: String) -> (bool, CacheContent) {
        if self.cache.contains_key(&fname) {
            let cc = self.cache[&fname].clone();
            self.cache.insert(fname.clone(), cc);
            //self.cache[&fname] =  ???     ??? why re assign old value to itself

            self.hitcnt += 1;
            return (true, self.cache[&fname].clone());
        }
        self.misscnt += 1;
        return (false, Default::default());
    }

    fn get(&self, fname: String) -> (bool, CacheContent) {
        assert!(self.cache.contains_key(&fname));
        return (true, self.cache[&fname].clone());
    }

    fn incref(&mut self, fname: String) {
        assert!(self.cache.contains_key(&fname));
        let cc = self.cache[&fname].clone();

        let newcc = CacheContent {
            refcnt: cc.refcnt + 1,
            ..cc
        };

        self.cache.insert(fname, newcc);
    }

    fn decref(&mut self, fname: String) {
        assert!(self.cache.contains_key(&fname));
        let cc = self.cache[&fname].clone();

        let newcc = CacheContent {
            refcnt: cc.refcnt - 1,
            ..cc
        };

        self.cache.insert(fname, newcc);
    }

    fn setdirty(&mut self, fname: String, dirty: bool) {
        assert!(self.cache.contains_key(&fname));
        let cc = self.cache[&fname].clone();

        let newcc = CacheContent { dirty: dirty, ..cc };

        self.cache.insert(fname, newcc);
    }

    fn setclean(&mut self, fname: String) {
        assert!(self.cache.contains_key(&fname));
        let cc = self.cache[&fname].clone();

        let newcc = CacheContent { dirty: false, ..cc };

        self.cache.insert(fname, newcc);
    }

    fn isdirty(&mut self, fname: String) -> bool {
        assert!(self.cache.contains_key(&fname));
        return self.cache[&fname].dirty == true;
    }

    fn setvalid(&mut self, fname: String) {
        assert!(self.cache.contains_key(&fname));
        let cc = self.cache[&fname].clone();

        let newcc = CacheContent { valid: true, ..cc };

        self.cache.insert(fname, newcc);
    }
}

#[derive(Clone)]
struct Client {
    name: String,
    cid: i32,
    server: Server,
    files: Files,
    bias: Vec<f64>,
    actions: String,
    solve: bool,
    detail: i32,
    cache: Cache,
    fd: FileDesc,
    readcnt: i32,
    writecnt: i32,
    done: bool,
    acnt: usize,
    acts3: Vec<(i32, String, i32)>,
    acts2: Vec<(i32, i32)>,
    am: Vec<(i32, usize)>,
}

static MICRO_OPEN: i32 = 1;
static MICRO_READ: i32 = 2;
static MICRO_WRITE: i32 = 3;
static MICRO_CLOSE: i32 = 4;

impl Client {
    fn new(
        name: String,
        cid: i32,
        server: Server,
        files: Files,
        bias: Vec<f64>,
        actions: String,
        solve: bool,
        detail: i32,
    ) -> Client {
        Client {
            name: name.clone(),
            cid: cid,
            server: server,
            files: files,
            bias: bias,
            actions: actions,
            solve: solve,
            detail: detail,
            cache: Cache::new(name, cid, solve, detail),
            fd: FileDesc::new(),
            readcnt: 0,
            writecnt: 0,
            done: false,
            acnt: 0,
            acts3: Vec::new(),
            acts2: Vec::new(),
            am: Vec::new(),
        }
    }

    fn init(&mut self, numsteps: i32) {
        let mut ai3 = 0;
        let mut ai2 = 0;
        self.fd.init();
        if self.actions == "" {
            for i in 0..numsteps {
                let fname = pickrandS(self.files.getfiles());
                let r: f64 = rand::thread_rng().gen();
                let fd = self.fd.alloc(fname.clone(), -1);
                assert!(
                    fd >= 0,
                    "client:init() -- ran out of file descriptors, sorry!"
                );
                if r < self.bias[0].into() {
                    self.acts3.push((MICRO_OPEN, fname, fd));
                    self.am.push((3, ai3));
                    ai3 += 1;
                    self.acts2.push((MICRO_READ, fd));
                    self.am.push((2, ai2));
                    ai2 += 1;
                    self.acts2.push((MICRO_CLOSE, fd));
                    self.am.push((2, ai2));
                    ai2 += 1;
                } else {
                    self.acts3.push((MICRO_OPEN, fname, fd));
                    self.am.push((3, ai3));
                    ai3 += 1;
                    self.acts2.push((MICRO_WRITE, fd));
                    self.am.push((2, ai2));
                    ai2 += 1;
                    self.acts2.push((MICRO_CLOSE, fd));
                    self.am.push((2, ai2));
                    ai2 += 1;
                }
            }
        } else {
            for a in self.actions.split(':') {
                let s = a.to_string();
                let c = &s[0..1];

                if a.starts_with("o") {
                    assert!(a.len() == 3, "client:init() -- malformed open action ({}) should be oa1 or something like that" , a);
                    let x = &s[1..2];
                    let y = &s[2..3];
                    let fname = x.to_string();
                    let fd: i32 = y.parse().unwrap();
                    assert!(fd >= 0);
                    self.acts3.push((MICRO_OPEN, fname, fd));
                    self.am.push((3, ai3));
                    ai3 += 1;
                } else if a.starts_with("r") {
                    assert!(a.len() == 2, "client:init() -- malformed open action ({}) should be r1 or something like that" , a);
                    let x = &s[1..2];
                    let fd: i32 = x.parse().unwrap();
                    self.acts2.push((MICRO_READ, fd));
                    self.am.push((2, ai2));
                    ai2 += 1;
                } else if a.starts_with("w") {
                    assert!(a.len() == 2, "client:init() -- malformed open action ({}) should be w1 or something like that" , a);
                    let x = &s[1..2];
                    let fd: i32 = x.parse().unwrap();
                    self.acts2.push((MICRO_WRITE, fd));
                    self.am.push((2, ai2));
                    ai2 += 1;
                } else if a.starts_with("c") {
                    assert!(a.len() == 2, "client:init() -- malformed open action ({}) should be c1 or something like that" , a);
                    let x = &s[1..2];
                    let fd: i32 = x.parse().unwrap();
                    self.acts2.push((MICRO_CLOSE, fd));
                    self.am.push((2, ai2));
                    ai2 += 1;
                } else {
                    println!("Unrecognized command: {} (from {})", c, a);
                }
            }
        }
    }

    fn setServer(&mut self, server: Server) {
        self.server = server;
    }

    fn getname(&self) -> String {
        return self.name.clone();
    }

    fn stats(&self) {
        println!(
            "{}       -- Reads:{} Writes:{}",
            self.name, self.readcnt, self.writecnt
        );
        self.cache.stats();
    }

    fn getfile(&mut self, fname: String, dirty: bool) {
        let (incache, item) = self.cache.checkget(fname.clone());
        if incache == true && item.valid == true {
            self.cache.setdirty(fname.clone(), dirty);
        } else {
            let data = self.server.get(self.name.clone(), fname.clone());
            self.cache.put(fname.clone(), data, dirty, 0);
        }
        self.cache.incref(fname.clone());
    }

    fn putfile(&mut self, fname: String, value: i32) {
        self.server.put(self.name.clone(), fname.clone(), value);
        self.cache.setclean(fname.clone());
        self.cache.setvalid(fname.clone());
    }

    fn invalidate(&mut self, fname: String) {
        self.cache.invalidate(fname);
    }

    fn step(&mut self, space: i32) -> i32 {
        if self.done == true {
            return -1;
        }
        if self.acnt == self.acts3.len() + self.acts2.len() {
            self.done = true;
            return 0;
        }

        let actions = self.am[self.acnt].0;
        let locate = self.am[self.acnt].1;

        //dospace(space);

        if isset(self.detail, 3) == true {
            print!("{}", self.name);
        }

        // now handle the action

        if actions == 3 {
            let fname = &self.acts3[locate].1;
            let fd = &self.acts3[locate].2;
            println!("open:{} [fd:{}]", fname, fd);
            self.getfile(fname.to_string(), false);
        } else {
            let actionsType = self.acts2[locate].0;

            if actionsType == MICRO_READ {
                let fd = &self.acts2[locate].1;
                let fname = self.fd.lookup(*fd);
                self.readcnt += 1;
                let (incache, contents) = self.cache.get(fname.clone());
                assert!(incache == true);
                if self.solve {
                    println!("read:{} -> {}", fd, contents.data);
                } else {
                    println!("read:{} -> value?", fd);
                }
            } else if actionsType == MICRO_WRITE {
                let fd = &self.acts2[locate].1;
                let fname = self.fd.lookup(*fd);
                self.writecnt += 1;
                let (incache, contents) = self.cache.get(fname.clone());
                assert!(incache == true);
                let v = self.files.getvalue();
                self.cache.update(fname, v);
                if self.solve {
                    println!("write:{} {} -> {}", fd, contents.data, v);
                } else {
                    println!("write:{} value? -> {}", fd, v);
                }
            } else if actionsType == MICRO_CLOSE {
                let fd = &self.acts2[locate].1;
                let fname = self.fd.lookup(*fd);
                let (incache, contents) = self.cache.get(fname.clone());
                assert!(incache == true);
                println!("close:{}", fd);
                if self.cache.isdirty(fname.clone()) {
                    self.putfile(fname.clone(), contents.data);
                }
                self.cache.decref(fname.clone());
                self.cache.checkvalid(fname.clone());
            }
        }

        // useful to see
        self.cache.printstate(self.name.clone());

        if self.solve && self.detail > 0 {
            println!("");
        }
        self.acnt += 1;
        return 1;
    }
}

struct AfsOption {
    seed: u64,
    numclients: i32,
    numsteps: i32,
    numfiles: i32,
    readratio: f64,
    actions: String,
    schedule: String,
    printstats: bool,
    solve: bool,
    detail: i32,
}

impl AfsOption {
    pub fn new() -> AfsOption {
        AfsOption {
            seed: 0,
            numclients: 2,
            numsteps: 2,
            numfiles: 1,
            readratio: 0.5,
            actions: String::from(""),
            schedule: String::from(""),
            printstats: false,
            solve: false,
            detail: 0,
        }
    }
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut afs_op = AfsOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" => {
                afs_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-C" => {
                afs_op.numclients = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-n" => {
                afs_op.numsteps = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-f" => {
                afs_op.numfiles = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-r" => {
                afs_op.readratio = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-A" => {
                afs_op.actions = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-S" => {
                afs_op.schedule = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-p" => {
                afs_op.printstats = true;
                i = i + 1;
            }
            "-c" => {
                afs_op.solve = true;
                i = i + 1;
            }
            "-d" => {
                afs_op.detail = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            _ => {
                println!("afs_op_parse match wrong!!");
                return;
            }
        }
    }
    execute_afs_op(afs_op);
}

fn execute_afs_op(options: AfsOption) {
    println!("ARG seed {}", options.seed);
    println!("ARG numclients {}", options.numclients);
    println!("ARG numsteps {}", options.numsteps);
    println!("ARG numfiles {}", options.numfiles);
    println!("ARG readratio {}", options.readratio);
    println!("ARG actions {}", options.actions);
    println!("ARG schedule {}", options.schedule);
    println!("ARG detail {}", options.detail);
    println!("");

    let seed = options.seed;
    let mut numclients = options.numclients;
    let numsteps = options.numsteps;
    let numfiles = options.numfiles;
    let readratio = options.readratio;
    let actions = options.actions;
    let schedule = options.schedule;
    let printstats = options.printstats;
    let solve = options.solve;
    let detail = options.detail;

    let mut _rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    assert!(
        numfiles > 0 && numfiles <= 26,
        "main: can only simulate 26 or fewer files, sorry"
    );
    assert!(
        readratio >= 0.0 && readratio <= 1.0,
        "main: read ratio must be between 0 and 1 inclusive"
    );

    let mut f = Files::new(numfiles);
    f.init();

    let mut s = Server::new(f.clone(), solve, detail);
    s.init();

    let mut clients: Vec<Client> = Vec::new();

    if actions != "" {
        let cactions: Vec<&str> = actions.split(',').collect();
        if numclients != cactions.len() as i32 {
            numclients = cactions.len() as i32;
        }
        let mut i = 0;

        for clist in cactions {
            let mut c = Client::new(
                format!("c{}", i),
                i,
                s.clone(),
                f.clone(),
                Vec::new(),
                clist.to_string(),
                solve,
                detail,
            );
            c.init(clist.len() as i32);
            clients.push(c);
            i += 1;
        }
    } else {
        for i in 0..numclients {
            let mut c = Client::new(
                format!("c{}", i),
                i,
                s.clone(),
                f.clone(),
                vec![readratio, 1.0],
                "".to_string(),
                solve,
                detail,
            );
            c.init(numsteps);
            clients.push(c);
        }
    }

    s.setclients(clients.clone());

    println!("          Server           ");
    for c in &clients {
        println!("             {}             ", c.getname());
    }
    println!();

    s.filestats(true);

    for c in &mut clients {
        c.setServer(s.clone());
    }

    let mut schedcurr = 0;

    if schedule != "" {
        for i in 0..clients.len() {
            let mut cnt = 0;
            for j in 0..schedule.len() {
                let curr = &schedule[j..j + 1];
                if curr.parse::<usize>().unwrap() == i {
                    cnt += 1;
                }
            }
            assert!(
                cnt != 0,
                "main: client {} not in schedule:{}, which would never terminate",
                i,
                schedule
            );
        }
    }

    let mut numrunning = clients.len();

    while numrunning > 0 {
        // let mut c;
        // if schedule == "" {
        //     c = pickrandC(clients.clone());
        // }else {
        //     let idx = &schedule[schedcurr..schedcurr+1].parse::<usize>().unwrap();
        //     c = clients[*idx].clone();
        //     schedcurr +=1;
        //     if schedcurr == schedule.len() {
        //         schedcurr = 0;
        //     }
        // }
        let mut c;
        if schedule == "" {
            c = pickrandC(&mut clients);
        } else {
            let idx = &schedule[schedcurr..schedcurr + 1].parse::<usize>().unwrap();
            c = &mut clients[*idx];
            schedcurr += 1;
            if schedcurr == schedule.len() {
                schedcurr = 0;
            }
        }

        //let rc = c.step(clients.index(c));
        let rc = c.step(5); //test

        if rc == 0 {
            numrunning -= 1;
        }
    }

    s.filestats(solve);

    if printstats {
        s.stats();
        for c in clients {
            c.stats()
        }
    }
}
