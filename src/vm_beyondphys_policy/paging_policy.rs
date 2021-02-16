// done
use rand::{Rng, SeedableRng};

const HELP: &str = include_str!("help.txt");

use std::collections::HashMap;

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

fn hfunc(index: i32) -> String {
    if index == -1 {
        return "MISS".to_string();
    } else {
        return "HIT".to_string();
    }
}

fn vfunc(victim: i32) -> String {
    if victim == -1 {
        return "-".to_string();
    } else {
        return victim.to_string();
    }
}

struct PPOption {
    seed: u64,
    cachesize: i32,
    addresses: String,
    address_file: String,
    policy: String,
    maxpage: i32,
    numaddrs: i32,
    clockbits: i32,
    notrace: bool,
    solve: bool,
}

impl PPOption {
    fn new() -> PPOption {
        PPOption {
            seed: 0,
            cachesize: 3,
            addresses: String::from("-1"),
            address_file: String::from(""),
            policy: String::from("FIFO"),
            maxpage: 10,
            numaddrs: 10,
            clockbits: 2,
            notrace: false,
            solve: false,
        }
    }
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut pp_op = PPOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" => {
                pp_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-a" => {
                pp_op.addresses = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-f" => {
                pp_op.address_file = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-p" => {
                pp_op.policy = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-C" => {
                pp_op.cachesize = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-m" => {
                pp_op.maxpage = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-n" => {
                pp_op.numaddrs = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-b" => {
                pp_op.clockbits = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-N" => {
                pp_op.notrace = true;
                i = i + 1;
            }
            "-c" => {
                pp_op.solve = true;
                i = i + 1;
            }
            _ => {
                println!("pp_op_parse match wrong!!");
                return;
            }
        }
    }
    execute_plt_op(pp_op);
}

fn execute_plt_op(options: PPOption) {
    println!("ARG seed : {}", options.seed);
    println!("ARG addresses : {}", options.addresses);
    println!("ARG address_file : {}", options.address_file);
    println!("ARG numaddrs : {}", options.numaddrs);
    println!("ARG policy : {}", options.policy);
    println!("ARG clockbits : {}", options.clockbits);
    println!("ARG cachesize : {}", options.cachesize);
    println!("ARG maxpage : {}", options.maxpage);
    println!("ARG notrace : {}", options.notrace);
    println!("");

    let addresses = options.addresses;
    let address_file = options.address_file;
    let numaddrs = options.numaddrs;
    let cachesize = options.cachesize;
    let seed = options.seed;
    let maxpage = options.maxpage;
    let policy = options.policy;
    let notrace = options.notrace;
    let clockbits = options.clockbits;

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    let mut addr_list: Vec<i32> = Vec::new();

    if address_file != "" {
        // open file
        println!("Not impl op file")
    } else {
        if addresses == "-1" {
            for _i in 0..numaddrs {
                let rand_x: f32 = rng.gen();
                let n = (maxpage as f32 * rand_x) as i32;
                addr_list.push(n);
            }
        } else {
            let addr_strlist: Vec<&str> = addresses.split(',').collect();
            for addr_str in addr_strlist {
                addr_list.push(addr_str.parse().unwrap());
            }
        }
    }

    if options.solve == false {
        println!(
            "Assuming a replacement policy of {}, and a cache of size {} pages,",
            policy, cachesize
        );
        println!("figure out whether each of the following page references hit or miss");
        println!("in the page cache.");

        for n in addr_list {
            println!("Access: {}  Hit/Miss?  State of Memory?", n);
        }

        println!("");
    } else {
        if notrace == false {
            println!("Solving....");
        }

        let mut count = 0;
        //let mut memory:Vec<i32> = vec![999]; // 999 eq null
        let mut memory: Vec<i32> = Vec::new();
        let mut hits = 0;
        let mut miss = 0;

        let mut _left_str = "";
        let mut _rite_str = "";
        if policy == "FIFO" {
            _left_str = "FirstIn";
            _rite_str = "Lastin";
        } else if policy == "LRU" {
            _left_str = "LRU";
            _rite_str = "MRU";
        } else if policy == "MRU" {
            _left_str = "LRU";
            _rite_str = "MRU";
        } else if policy == "OPT" || policy == "RAND" || policy == "UNOPT" || policy == "CLOCK" {
            _left_str = "Left";
            _rite_str = "Right";
        } else {
            println!("Policy {} is not yet implemented", policy);
            return;
        }

        let mut refer: HashMap<i32, i32> = HashMap::new();

        let cdebug = false;

        let mut addr_index = 0;
        for n in &addr_list {
            let mut idx = -1;

            for i in 0..memory.len() {
                //memory
                if memory[i] == *n {
                    idx = i as i32;
                    hits += 1;
                    if policy == "LRU" || policy == "MRU" {
                        memory.remove(i);
                        memory.push(*n);
                    }
                    break;
                } else {
                    idx = -1;
                }
            }
            if idx == -1 {
                miss += 1;
            }

            let mut victim = -1;
            if idx == -1 {
                if count == cachesize {
                    // if count == 0  crash !!!
                    if policy == "FIFO" || policy == "LRU" {
                        victim = memory.remove(0);
                    } else if policy == "MRU" {
                        victim = memory.remove((count - 1) as usize)
                    } else if policy == "RAND" {
                        let rand_y: f32 = rng.gen();
                        victim = memory.remove((rand_y * count as f32) as usize)
                    } else if policy == "CLOCK" {
                        if cdebug {
                            println!("REFERENCE TO PAGE : {}", n);
                            println!("MEMORY : {:?}", memory);
                            println!("REF(b) : {:?}", refer);
                        }
                        victim = -1;
                        while victim == -1 {
                            let ranz: f32 = rng.gen();
                            let page = memory[(ranz * count as f32) as usize];
                            if cdebug {
                                println!("  scan page:{}  {}", page, refer[&page]);
                            }
                            if refer[&page] >= 1 {
                                let tmp = refer[&page];
                                refer.insert(page, tmp - 1);
                            } else {
                                victim = page;
                                for z in 0..memory.len() {
                                    if memory[z as usize] == page {
                                        memory.remove(z as usize);
                                    }
                                    break;
                                }
                                break;
                            }
                        }

                    // No GC how to do del
                    // if page in memory:
                    //     assert('BROKEN')
                    // del ref[victim]
                    // if cdebug:
                    //     print 'VICTIM', page
                    //     print 'LEN', len(memory)
                    //     print 'MEM', memory
                    //     print 'REF (a)', ref
                    } else if policy == "OPT" {
                        let mut max_replace = 0;
                        let mut replace_idx = -1;
                        let mut _replace_page = -1;

                        for page_index in 0..count {
                            let page = memory[page_index as usize];
                            let mut when_referenced = addr_list.len();
                            for future_idx in addr_index + 1..addr_list.len() {
                                let future_page = addr_list[future_idx];
                                if page == future_page {
                                    when_referenced = future_idx;
                                    break;
                                }
                            }

                            if when_referenced >= max_replace {
                                replace_idx = page_index;
                                _replace_page = page;
                                max_replace = when_referenced;
                            }
                        }

                        victim = memory.remove(replace_idx as usize);
                    } else if policy == "UNOPT" {
                        let mut min_replace = addr_list.len() + 1;
                        let mut replace_idx = -1;
                        let mut _replace_page = -1;
                        for page_index in 0..count {
                            let page = memory[page_index as usize];
                            let mut when_referenced = addr_list.len();
                            for future_idx in addr_index + 1..addr_list.len() {
                                let future_page = addr_list[future_idx];
                                if page == future_page {
                                    when_referenced = future_idx;
                                    break;
                                }
                            }

                            if when_referenced < min_replace {
                                replace_idx = page_index;
                                _replace_page = page;
                                min_replace = when_referenced;
                            }
                        }
                        victim = memory.remove(replace_idx as usize);
                    }
                } else {
                    victim = -1;
                    count = count + 1;
                }

                memory.push(*n);

                if cdebug {
                    println!("LEN(a) : {} ", memory.len());
                }
                if victim != -1 {
                    for vi in &memory {
                        if victim == *vi {
                            println!("ok victim is not in memory now  {}", victim);
                        }
                    }
                }
            }

            let ct = refer.entry(*n).or_insert(1);
            *ct += 1;
            if *ct > clockbits {
                *ct = clockbits;
            }

            if cdebug {
                println!("REF (a) : {:?}", refer);
            }

            if notrace == false {
                println!(
                    "Access: {}  {}  {} ->  {:?}  <- {} Replaced:{} [Hits:{} Misses:{}]",
                    n,
                    hfunc(idx),
                    _left_str,
                    &memory[0..],
                    _rite_str,
                    vfunc(victim),
                    hits,
                    miss
                );
            }

            addr_index = addr_index + 1;
        }

        println!("");
        println!(
            "FINALSTATS hits {}   misses {}   hitrate  : {hitrate}",
            hits,
            miss,
            hitrate = format!("{:.*}", 2, (100.0 * hits as f32) / (hits + miss) as f32)
        );
        println!("");
    }
}
