// done except sort

use std::collections::HashMap;
use rand::prelude::*;
struct Malloc {
    size:i32,
    headerSize:i32,
    freelist:Vec<(i32,i32)>,
    sizemap:HashMap<i32,i32>,
    policy:String,
    returnPolicy:String,
    coalesce:bool,
    align:i32,
    newlist:Vec<(i32,i32)>,
    curr:(i32,i32),
}

impl Malloc {
    fn new(size:i32,start:i32,headerSize:i32,policy:String,order:String,coalesce:bool,align:i32) -> Malloc {
        Malloc {
            size:size,
            headerSize:headerSize,
            freelist:vec![(start,size)],
            sizemap:HashMap::new(),
            policy:policy,
            returnPolicy:order,
            coalesce:coalesce,
            align:align,
            newlist:vec![(0,0)],
            curr:(0,0),
        }
    }

    fn addToMap(&mut self,addr:i32,size:i32) {
        self.sizemap.entry(addr).or_insert(size);
    }

    fn malloc(&mut self,mut size:i32)->(i32,i32) {
        if self.align != -1 {
            let left = size % self.align;
            let mut diff = 0;
            if left != 0 {
                diff = self.align - left;
            }else {
                diff = 0;
            }
            size += diff;
        }
        size += self.headerSize;

        let mut bestIdx:usize = 999999999;              // because usize can not be -1 use 99 instead
        let mut bestSize = 0;
        if self.policy == "BEST" {
            bestSize = self.size+1;
        }else if self.policy == "WORST" || self.policy == "FIRST" {
            bestSize = -1;
        }

        let mut count = 0;
        let mut bestAddr = 0;

        for i in 0..self.freelist.len() {
            let eaddr = self.freelist[i].0;
            let esize = self.freelist[i].1;
            count += 1;

            if esize >= size && ((self.policy == "BEST" && esize <bestSize) ||
                                                    (self.policy == "WORST" && esize > bestSize) ||
                                                    (self.policy == "FIRST")) 
            {
                bestAddr = eaddr;
                bestSize = esize;
                bestIdx = i;
                if self.policy == "FIRST"
                {
                    break;
                }
            }
        }

        if bestIdx != 999999999 {
            if bestSize > size {
                self.freelist[bestIdx] = (bestAddr +size,bestSize - size);
                self.addToMap(bestAddr, size);
            }else if bestSize == size {
                self.freelist.remove(bestIdx);
                self.addToMap(bestAddr, size);
            }else {
                // ???   abort??
            }
            return (bestAddr,count);
        }

        return (-1,count);
    }

    fn free(&mut self,addr:i32) -> i32 {
        if self.sizemap.contains_key(&addr) {
            return -1;
        }

        
        let size = self.sizemap[&addr];

        if self.returnPolicy == "INSERT-BACK" {
            self.freelist.push((addr,size));
        }else if self.returnPolicy == "INSERT-FRONT" {
            self.freelist.insert(0, (addr, size));
        }else if self.returnPolicy == "ADDRSORT" {
            self.freelist.push((addr, size));
            // sort
        }else if self.returnPolicy == "SIZESORT+" {
            self.freelist.push((addr,size));
            // sort
        }else if self.returnPolicy == "SIZESORT-" {
            self.freelist.push((addr,size));
            // sort
        }

        if self.coalesce == true {
            self.newlist.clear();
            self.curr = self.freelist[0];
            for i in 1..self.freelist.len() {
                let eaddr = self.freelist[i].0;
                let esize = self.freelist[i].1;
                if eaddr == (self.curr.0+self.curr.1) {
                    self.curr = (self.curr.0, self.curr.1 + esize);
                }else {
                    self.newlist.push(self.curr);
                    self.curr = (eaddr,esize);
                }
            }
            self.newlist.push(self.curr);
            self.freelist.clear();
            for t in &self.newlist {
                self.freelist.push(*t);
            }
            //self.freelist = self.newlist;
        }
        return 0;
    }

    fn dump(&self) {
        println!("Free List [ Size {} ] : ",self.freelist.len());
        for e in &self.freelist {
            println!("[ addr:{} sz:{} ]",(*e).0,(*e).1);
        }
        println!("");
    }
}

struct MallocOption {
    seed:i32,
    heapSize:i32,
    baseAddr:i32,
    headerSize:i32,
    alignment:i32,
    policy:String,
    order:String,
    coalesce:bool,
    opsNum:i32,
    opsRange:i32,
    opsPAlloc:i32,
    opsList:String,
    solve:bool,
}

impl MallocOption {
    fn new() -> MallocOption {
        MallocOption {
            seed:0,
            heapSize:100,
            baseAddr:1000,
            headerSize:0,
            alignment:-1,
            policy:String::from("BEST"),
            order:String::from("ADDRSORT"),
            coalesce:false,
            opsNum:10,
            opsRange:10,
            opsPAlloc:50,
            opsList:String::from(""),
            solve:false,
        }
    }
}

pub fn malloc_op_parse(op_vec:Vec<&str>) {
    let mut mall_op = MallocOption::new();
    let mut i =1;
    while i<op_vec.len() {
        match op_vec[i] {
            "-s" =>{mall_op.seed = op_vec[i+1].parse().unwrap();i = i+2;},
            "-S" =>{mall_op.heapSize = op_vec[i+1].parse().unwrap();i = i+2;},
            "-b" =>{mall_op.baseAddr = op_vec[i+1].parse().unwrap();i=i+2;},
            "-H" =>{mall_op.headerSize = op_vec[i+1].parse().unwrap();i=i+2;},
            "-a" =>{mall_op.alignment = op_vec[i+1].parse().unwrap();i=i+2;},
            "-p" =>{mall_op.policy = op_vec[i+1].to_string();i=i+2;},
            "-l" =>{mall_op.order = op_vec[i+1].to_string();i=i+2;},
            "-C" =>{mall_op.coalesce = true;i=i+1;},
            "-n" =>{mall_op.opsNum = op_vec[i+1].parse().unwrap();i = i+2;},
            "-r" =>{mall_op.opsRange = op_vec[i+1].parse().unwrap();i=i+2;},
            "-P" =>{mall_op.opsPAlloc = op_vec[i+1].parse().unwrap();i=i+2;},
            "-A" =>{mall_op.opsList = op_vec[i+1].to_string();i=i+2;},
            "-c" =>{mall_op.solve = true;i=i+1;},
            _ => println!("mall_op_parse match wrong!!"),
        }
    }
    execute_malloc_op(mall_op);
}

fn execute_malloc_op(options:MallocOption) {

    let seed_u8 = options.seed as u8;
    let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

    let mut rng = SmallRng::from_seed(seed);

    let mut m = Malloc::new(options.heapSize, options.baseAddr, options.headerSize,
           options.policy, options.order, options.coalesce, options.alignment);

    let percent = (options.opsPAlloc as f32 / 100f32);

    let mut P:HashMap<i32,i32> = HashMap::new();
    let mut L:Vec<i32> = Vec::new();

    assert!(percent>0f32);

    if options.opsList == "" {
        let mut c = 0;
        let mut j = 0;

        while j<options.opsNum {
            let mut pr = false;
            let rand_x:f32 = rng.gen();
            if rand_x < percent{ 
                let rand_y:f32 = rng.gen();
                let size = (rand_y * options.opsRange as f32 ) as i32 +1;
                let (ptr,cnt) = m.malloc(size);

                if ptr != -1 {
                    P.insert(c,ptr);
                    L.push(c);
                }
                println!("ptr[{}] = Alloc({})" ,c, size);

                if options.solve == true {
                    println!(" returned {} (searched {} elements) ",ptr+options.headerSize,cnt);
                }else {
                    println!("returned ?");
                }

                c+=1;
                j+=1;
                pr = true;
            }else {
                if P.len() > 0 {
                    let rand_z:f32 = rng.gen();
                    let d = (rand_z * L.len() as f32) as usize;

                    let rc = m.free(P[&L[d]]);

                    println!("Free(ptr[{}])",L[d]);

                    if options.solve == true {
                        println!("returned {}" ,rc);
                    }else {
                        println!("returned ?");
                    }

                    pr = true;
                    j+=1;
                }
            }

            if pr {
                if options.solve {
                    m.dump();
                }else {
                    println!("List ?");
                }
            }
        }
    }else{
        // let mut c = 0;
        // for op in options.opsList.split(",").collect() {
        //     if op[0] == "+" {
        //         let mut size = op.split("-").collect()[1];
        //         let (ptr, cnt) = m.malloc(size);
        //         if ptr != -1 {
        //             P.insert(c,ptr);
        //         }
        //         println!("ptr[{}] = Alloc({})" ,c, size);

        //         if options.solve == true {
        //             println!(" returned {} (searched {} elements) ",ptr,cnt);
        //         }else {
        //             println!("returned ?");
        //         }

        //         c+=1;
        //     }else if op[0] == "-" {
        //         let mut index = op.split("-").collect()[1];
        //         if index>= P.len() {
        //             println!("Invalid Free: Skipping");
        //             continue;
        //         }
        //         print!("Free(ptr[{}])",index);
        //         let rc = m.free(p[index]);
        //         if options.solve == true {
        //                 println!("returned {}" ,rc);
        //             }else {
        //                 println!("returned ?");
        //             }
        //     }else {
        //         println!("badly specified operand: must be +Size or -Index");
        //     }

        //     if options.solve == true {
        //         m.dump();
        //     }else {
        //         println!("List ?")
        //     }
        // }
    }
}