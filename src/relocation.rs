//  done

use rand::prelude::*;
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

struct RelocationOption {
    seed:i32,
    asize:String,
    psize:String,
    num:i32,
    base:String,
    limit:String,
    solve:bool,
}

impl RelocationOption {
    pub fn new() -> RelocationOption {
        RelocationOption {
            seed:0,
            asize:String::from("1k"),
            psize:String::from("16k"),
            num:5,
            base:String::from("-1"),
            limit:String::from("-1"),
            solve:false,
        }
    }
}

pub fn relocation_op_parse(op_vec:Vec<&str>) {
        let mut relo_op = RelocationOption::new();
        let mut i =1;
        while i<op_vec.len() {
            match op_vec[i] {
                "-s" =>{relo_op.seed = op_vec[i+1].parse().unwrap();i = i+2;},
                "-a" =>{relo_op.asize = op_vec[i+1].to_string();i = i+2;},
                "-p" =>{relo_op.psize = op_vec[i+1].to_string();i=i+2;},
                "-n" =>{relo_op.num = op_vec[i+1].parse().unwrap();i=i+2;},
                "-b" =>{relo_op.base = op_vec[i+1].to_string();;i=i+2;},
                "-l" =>{relo_op.limit = op_vec[i+1].to_string();;i=i+2;},
                "-c" =>{relo_op.solve = true;i=i+1;},
                _ => println!("relocation_op_parse match wrong!!"),
            }
        }
        execute_relocation_op(relo_op);
}

fn execute_relocation_op(options:RelocationOption) {
    println!("");
    println!("ARG seed : {} ", options.seed);
    println!("ARG address space size : {} ", options.asize);
    println!("ARG phys mem size : {} ",options.psize);
    println!("");

    let seed_u8 = options.seed as u8;
    let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

    let mut rng = SmallRng::from_seed(seed);

    let asize = convert(options.asize);
    let psize = convert(options.psize);

    if psize<=1 {
        println!("Error: must specify a non-zero physical memory size.");
        return
    }

    if asize == 0 {
        println!("Error: must specify a non-zero address-space size.");
        return
    }

    if psize <= asize {
        println!("Error: physical memory size must be GREATER than address space size (for this simulation)");
        return 
    }

    let mut limit = convert(options.limit);
    let mut base = convert(options.base);

    if limit == -1 {
        let rand_x:f32 = rng.gen();
        limit = ((asize as f32 /4.0) + (asize as f32/4.0 * rand_x)) as i32;
    }

    if base == -1 {
        // let mut done = 0;
        // while done==0 {
        //     //base = 
        //     if (base+limit) < psize {
        //         done = 1;
        //     }
        // }
        loop {
            let rand_x:f32 = rng.gen();
            base = (psize as f32 * rand_x) as i32;
             if (base+limit) < psize {
                 break;
             }
        }
    }

    println!("Base-and-Bounds register information:");
    println!("");
    println!("  Base   :  0x{:x}   (decimal {} )",base, base);
    println!("  Limit  :   {} ", limit);
    println!("");

    if (base+limit) > psize {
        println! ("Error: address space does not fit into physical memory with those base/bounds values.");
        println! ("Base + Limit: {}  Psize: {} ", base + limit,psize);
        return
    }

    println!("Virtual Address Trace");
    for i in 0..options.num {
        let rand_x:f32 = rng.gen();
        let mut vaddr = (asize as f32 * rand_x) as i32;
        if options.solve == false {
            println!("  VA  {} : 0x{:x} (decimal : {} )--> PA or segmentation violation?",i, vaddr, vaddr);
        }else {
            let mut paddr = 0;
            if vaddr>=limit {
                println!("  VA  {} : 0x{:x} (decimal : {} ) --> SEGMENTATION VIOLATION",i, vaddr, vaddr);
            }else{
                paddr = vaddr + base;
                println!("  VA  {} : 0x{:x} (decimal : {} ) --> VALID: 0x{:x} (decimal: {})" ,i, vaddr, vaddr, paddr, paddr);
            }
        }
    }

    println!("");

    if options.solve == false {
        println!("For each virtual address, either write down the physical address it translates to");
        println!("OR write down that it is an out-of-bounds address (a segmentation violation). For");
        println!("this problem, you should assume a simple virtual address space of a given size.");
        println!("");
    }
}