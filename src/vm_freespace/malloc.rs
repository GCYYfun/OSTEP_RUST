// done except sort

use rand::{Rng, SeedableRng};
use std::collections::HashMap;
const HELP: &str = include_str!("help.txt");
struct Malloc {
    size: i32,
    header_size: i32,
    freelist: Vec<(i32, i32)>,
    sizemap: HashMap<i32, i32>,
    policy: String,
    return_policy: String,
    coalesce: bool,
    align: i32,
    newlist: Vec<(i32, i32)>,
    curr: (i32, i32),
}

impl Malloc {
    fn new(
        size: i32,
        start: i32,
        header_size: i32,
        policy: String,
        order: String,
        coalesce: bool,
        align: i32,
    ) -> Malloc {
        Malloc {
            size: size,
            header_size: header_size,
            freelist: vec![(start, size)],
            sizemap: HashMap::new(),
            policy: policy,
            return_policy: order,
            coalesce: coalesce,
            align: align,
            newlist: vec![(0, 0)],
            curr: (0, 0),
        }
    }

    fn add_to_map(&mut self, addr: i32, size: i32) {
        assert!(!self.sizemap.contains_key(&addr));
        self.sizemap.insert(addr,size);
    }

    fn malloc(&mut self, mut size: i32) -> (i32, i32) {
        if self.align != -1 {
            let left = size % self.align;
            let mut _diff = 0;
            if left != 0 {
                _diff = self.align - left;
            } else {
                _diff = 0;
            }
            size += _diff;
        }
        size += self.header_size;

        let mut best_idx: usize = 999999999; // because usize can not be -1 use 99 instead
        let mut best_size = 0;
        if self.policy == "BEST" {
            best_size = self.size + 1;
        } else if self.policy == "WORST" || self.policy == "FIRST" {
            best_size = -1;
        }

        let mut count = 0;
        let mut best_addr = 0;

        for i in 0..self.freelist.len() {
            let eaddr = self.freelist[i].0;
            let esize = self.freelist[i].1;
            count += 1;

            if esize >= size
                && ((self.policy == "BEST" && esize < best_size)
                    || (self.policy == "WORST" && esize > best_size)
                    || (self.policy == "FIRST"))
            {
                best_addr = eaddr;
                best_size = esize;
                best_idx = i;
                if self.policy == "FIRST" {
                    break;
                }
            }
        }

        if best_idx != 999999999 {
            if best_size > size {
                self.freelist[best_idx] = (best_addr + size, best_size - size);
                self.add_to_map(best_addr, size);
            } else if best_size == size {
                self.freelist.remove(best_idx);
                self.add_to_map(best_addr, size);
            } else {
                panic!("should never get here");
            }
            return (best_addr, count);
        }

        return (-1, count);
    }

    fn free(&mut self, addr: i32) -> i32 {
        if !self.sizemap.contains_key(&addr) {
            return -1;
        }

        let size = self.sizemap[&addr];

        if self.return_policy == "INSERT-BACK" {
            self.freelist.push((addr, size));
        } else if self.return_policy == "INSERT-FRONT" {
            self.freelist.insert(0, (addr, size));
        } else if self.return_policy == "ADDRSORT" {
            self.freelist.push((addr, size));
            self.freelist.sort_by(|a,b| a.0.cmp(&b.0));
        } else if self.return_policy == "SIZESORT+" {
            self.freelist.push((addr, size));
            self.freelist.sort_by(|a,b| a.1.cmp(&b.1));
        } else if self.return_policy == "SIZESORT-" {
            self.freelist.push((addr, size));
            self.freelist.sort_by(|a,b| b.1.cmp(&a.1));
        }

        if self.coalesce == true {
            self.newlist.clear();
            self.curr = self.freelist[0];
            for i in 1..self.freelist.len() {
                let eaddr = self.freelist[i].0;
                let esize = self.freelist[i].1;
                if eaddr == (self.curr.0 + self.curr.1) {
                    self.curr = (self.curr.0, self.curr.1 + esize);
                } else {
                    self.newlist.push(self.curr);
                    self.curr = (eaddr, esize);
                }
            }
            self.newlist.push(self.curr);
            // self.freelist.clear();
            // for t in &self.newlist {
            //     self.freelist.push(*t);
            // }
            self.freelist = self.newlist.clone();
        }
        self.sizemap.remove(&addr);
        return 0;
    }

    fn dump(&self) {
        print!("Free List [ Size {} ] : ", self.freelist.len());
        for e in &self.freelist {
            print!("[ addr:{} sz:{} ]", (*e).0, (*e).1);
        }
        println!("");
    }
}

struct MallocOption {
    seed: u64,
    heap_size: i32,
    base_addr: i32,
    header_size: i32,
    alignment: i32,
    policy: String,
    order: String,
    coalesce: bool,
    ops_num: i32,
    ops_range: i32,
    ops_p_alloc: i32,
    ops_list: String,
    solve: bool,
}

impl MallocOption {
    fn new() -> MallocOption {
        MallocOption {
            seed: 1,
            heap_size: 100,
            base_addr: 1000,
            header_size: 0,
            alignment: -1,
            policy: String::from("BEST"),
            order: String::from("ADDRSORT"),
            coalesce: false,
            ops_num: 10,
            ops_range: 10,
            ops_p_alloc: 50,
            ops_list: String::from(""),
            solve: false,
        }
    }
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut mall_op = MallocOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" => {
                mall_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-S" => {
                mall_op.heap_size = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-b" => {
                mall_op.base_addr = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-H" => {
                mall_op.header_size = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-a" => {
                mall_op.alignment = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-p" => {
                mall_op.policy = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-l" => {
                mall_op.order = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-C" => {
                mall_op.coalesce = true;
                i = i + 1;
            }
            "-n" => {
                mall_op.ops_num = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-r" => {
                mall_op.ops_range = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-P" => {
                mall_op.ops_p_alloc = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-A" => {
                mall_op.ops_list = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-c" => {
                mall_op.solve = true;
                i = i + 1;
            }
            _ => {
                println!("mall_op_parse match wrong!!");
                return;
            }
        }
    }
    execute_malloc_op(mall_op);
}

fn execute_malloc_op(options: MallocOption) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(options.seed);

    let mut m = Malloc::new(
        options.heap_size,
        options.base_addr,
        options.header_size,
        options.policy,
        options.order,
        options.coalesce,
        options.alignment,
    );

    let percent = options.ops_p_alloc as f32 / 100f32;

    let mut p: HashMap<i32, i32> = HashMap::new();
    let mut l: Vec<i32> = Vec::new();

    assert!(percent > 0f32);

    if options.ops_list == "" {
        let mut c = 0;
        let mut j = 0;

        while j < options.ops_num {
            let mut pr = false;
            let rand_x: f32 = rng.gen();
            if rand_x < percent {
                let rand_y: f32 = rng.gen();
                let size = (rand_y * options.ops_range as f32) as i32 + 1;
                let (ptr, cnt) = m.malloc(size);

                if ptr != -1 {
                    p.insert(c, ptr);
                    l.push(c);
                }
                print!("ptr[{}] = Alloc({})", c, size);

                if options.solve == true {
                    println!(
                        " returned {} (searched {} elements) ",
                        ptr + options.header_size,
                        cnt
                    );
                } else {
                    println!("returned ?");
                }

                c += 1;
                j += 1;
                pr = true;
            } else {
                if p.len() > 0 {
                    let rand_z: f32 = rng.gen();
                    let d = (rand_z * l.len() as f32) as usize;

                    let rc = m.free(p[&l[d]]);

                    println!("Free(ptr[{}])", l[d]);

                    if options.solve == true {
                        println!("returned {}", rc);
                    } else {
                        println!("returned ?");
                    }

                    pr = true;
                    j += 1;
                }
            }

            if pr {
                if options.solve {
                    m.dump();
                } else {
                    println!("List ?");
                }
            }
            println!("")
        }
    } else {
        let mut c = 0;
        for op in options.ops_list.split(",").collect::<Vec<&str>>() {
            if op.starts_with("+") {
                let size = op.split("+").collect::<Vec<&str>>()[1].parse().unwrap();
                let (ptr, cnt) = m.malloc(size);
                if ptr != -1 {
                    p.insert(c,ptr);
                }
                println!("ptr[{}] = Alloc({})" ,c, size);

                if options.solve == true {
                    println!(" returned {} (searched {} elements) ",ptr,cnt);
                }else {
                    println!("returned ?");
                }

                c+=1;
            }else if op.starts_with("-") {
                let index = op.split("-").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
                if index>= p.len() {
                    println!("Invalid Free: Skipping");
                    continue;
                }
                print!("Free(ptr[{}])",index);
                let rc = m.free(p[&(index as i32)]);
                if options.solve == true {
                        println!("returned {}" ,rc);
                    }else {
                        println!("returned ?");
                    }
            }else {
                panic!("badly specified operand: must be +Size or -Index");
            }

            if options.solve == true {
                m.dump();
            }else {
                println!("List ?")
            }
            println!("");
        }
    }
}
