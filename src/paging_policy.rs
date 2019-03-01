// done
use rand::prelude::*;
use std::collections::HashMap;

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

fn hfunc(index:i32) ->String{
    if index == -1 {
        return "MISS".to_string();
    }else {
        return "HIT".to_string();
    }
}

fn vfunc(victim:i32) ->String{
    if victim == -1 {
        return "-".to_string();
    }else {
        return victim.to_string();
    }
}

struct PPOption {
    seed:i32,
    cachesize:i32,
    addresses:String,
    addressfile:String,
    policy:String,
    maxpage:i32,
    numaddrs:i32,
    clockbits:i32,
    notrace:bool,
    solve:bool,
}

impl PPOption {
    fn new() -> PPOption {
        PPOption {
            seed:0,
            cachesize:3,
            addresses:String::from("-1"),
            addressfile:String::from(""),
            policy:String::from("FIFO"),
            maxpage:10,
            numaddrs:10,
            clockbits:2,
            notrace:false,
            solve:false,
        }
    }
}

pub fn pp_op_parse(op_vec:Vec<&str>) {
        let mut pp_op = PPOption::new();
        let mut i =1;
        while i<op_vec.len() {
            match op_vec[i] {
                "-s" =>{pp_op.seed = op_vec[i+1].parse().unwrap();i = i+2;},
                "-a" =>{pp_op.addresses = op_vec[i+1].to_string();i = i+2;},
                "-f" =>{pp_op.addressfile = op_vec[i+1].to_string();i = i+2;},
                "-p" =>{pp_op.policy = op_vec[i+1].to_string();i=i+2;},
                "-C" =>{pp_op.cachesize = op_vec[i+1].parse().unwrap();i=i+2;},
                "-m" =>{pp_op.maxpage = op_vec[i+1].parse().unwrap();i=i+2;},
                "-n" =>{pp_op.numaddrs = op_vec[i+1].parse().unwrap();i=i+2;},
                "-b" =>{pp_op.clockbits = op_vec[i+1].parse().unwrap();i=i+2;},
                "-N" =>{pp_op.notrace = true;i=i+1;},
                "-c" =>{pp_op.solve = true;i=i+1;},
                _ => println!("pp_op_parse match wrong!!"),
            }
        }
        execute_plt_op(pp_op);
}

fn execute_plt_op(options:PPOption) {
    let seed_u8 = options.seed as u8;
    let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

    let mut rng = SmallRng::from_seed(seed);

    println! ("ARG seed : {}",               options.seed);
    println! ("ARG addresses : {}", options.addresses);
    println! ("ARG addressfile : {}",      options.addressfile);
    println! ("ARG numaddrs : {}",          options.numaddrs);
    println! ("ARG policy : {}",            options.policy);
    println! ("ARG clockbits : {}",          options.clockbits);
    println! ("ARG cachesize : {}",          options.cachesize);
    println! ("ARG maxpage : {}",          options.maxpage);
    println! ("ARG notrace : {}",          options.notrace);
    println! ("");

    let addresses   = options.addresses;
    let addressFile = options.addressfile;
    let numaddrs    = options.numaddrs;
    let cachesize   = options.cachesize;
    let seed        = options.seed;
    let maxpage     = options.maxpage;
    let policy      = options.policy;
    let notrace     = options.notrace;
    let clockbits   = options.clockbits;

    let mut addrList:Vec<i32> = Vec::new();

    if addressFile !="" {
        // open file
        println!("Not impl op file")
    }else {
        if addresses == "-1" {
            for i in 0..numaddrs {
                let rand_x:f32 = rng.gen();
                let n = (maxpage as f32 * rand_x) as i32;
                addrList.push(n);
            }
        }else {
            let addrStrList:Vec<&str>= addresses.split(',').collect();
            for addrStr in addrStrList {
                addrList.push(addrStr.parse().unwrap());
            }
        }
    }

    if options.solve ==false {
        println!("Assuming a replacement policy of {}, and a cache of size {} pages,",policy,cachesize);
        println!("figure out whether each of the following page references hit or miss");
        println!("in the page cache.");

        for n in addrList {
            println!("Access: {}  Hit/Miss?  State of Memory?",n);
        }

        println!("");
    }else{
        if notrace == false {
            println!("Solving....");
        }

        let mut count = 0;
        //let mut memory:Vec<i32> = vec![999]; // 999 eq null
        let mut memory:Vec<i32> = Vec::new();
        let mut hits = 0;
        let mut miss = 0;

        let mut leftStr = "";
        let mut riteStr = "";
        if policy == "FIFO" {
            leftStr = "FirstIn";
            riteStr = "Lastin";
        }else if policy == "LRU" {
            leftStr = "LRU";
            riteStr = "MRU";
        }else if policy == "MRU" {
            leftStr = "LRU";
            riteStr = "MRU";
        }else if policy == "OPT" || policy == "RAND" || policy == "UNOPT" || policy == "CLOCK"{
            leftStr = "Left";
            riteStr = "Right";
        }else{
            println!("Policy {} is not yet implemented",policy);
            return;
        }

        let mut refer:HashMap<i32,i32> = HashMap::new();

        let cdebug = false;

        let mut addrIndex = 0;
        for n in &addrList {
            let mut idx = -1;

            for i in 0..memory.len() {
                //memory
                if memory[i] == *n {
                    idx = i as i32;
                    hits +=1;
                    if policy == "LRU" || policy == "MRU" {
                        memory.remove(i);
                        memory.push(*n);
                    }
                    break;
                }else {
                    idx = -1;
                }
            }
            if idx == -1 {
                miss+=1;
            }

            let mut victim = -1;
            if idx == -1{
                if count == cachesize {   // if count == 0  crash !!!
                    if policy == "FIFO" || policy == "LRU" {
                        victim = memory.remove(0);
                    }else if policy == "MRU" {
                        victim = memory.remove((count-1) as usize)
                    }else if policy == "RAND" {
                        let rand_y:f32 = rng.gen();
                        victim = memory.remove((rand_y * count as f32) as usize)
                    }else if policy == "CLOCK" {
                        if cdebug {
                            println!("REFERENCE TO PAGE : {}" , n);
                            println!("MEMORY : {:?}", memory);
                            println!("REF(b) : {:?}",refer);
                        }
                        victim =-1;
                        while victim == -1 {
                            let ranz:f32 = rng.gen();
                            let page = memory[(ranz*count as f32) as usize];
                            if cdebug {
                                println!("  scan page:{}  {}", page, refer[&page]);
                            }
                            if refer[&page] >= 1 {
                                let tmp = refer[&page];
                                refer.insert(page, tmp-1);
                            }else {
                                victim = page;
                                for z in 0..memory.len(){
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
                    }else if policy == "OPT" {
                        let mut maxReplace = 0;
                        let mut replaceIdx = -1;
                        let mut replacePage = -1;

                        
                        for pageIndex in 0..count {
                            let page = memory[pageIndex as usize];
                            let mut whenReferenced = addrList.len();
                            for futureIdx in addrIndex+1..addrList.len() {
                                let futurePage = addrList[futureIdx];
                                if page == futurePage {
                                    whenReferenced = futureIdx;
                                    break;
                                }
                            }

                            if whenReferenced >= maxReplace {
                                replaceIdx = pageIndex;
                                replacePage = page;
                                maxReplace = whenReferenced;
                            }
                        }

                        victim = memory.remove(replaceIdx as usize);
                  
                    }else if policy == "UNOPT" {
                        let mut minReplace = addrList.len() +1;
                        let mut replaceIdx = -1;
                        let mut replacePage = -1;
                        for pageIndex in 0..count {
                            let page = memory[pageIndex as usize];
                            let mut whenReferenced = addrList.len();
                            for futureIdx in addrIndex+1..addrList.len() {
                                let futurePage = addrList[futureIdx];
                                if page == futurePage {
                                    whenReferenced = futureIdx;
                                    break;
                                }
                            }

                            if whenReferenced < minReplace {
                                replaceIdx = pageIndex;
                                replacePage = page;
                                minReplace = whenReferenced;
                            }

                        }
                        victim = memory.remove(replaceIdx as usize);
                    }
                }else {
                    victim = -1;
                    count = count+1;
                }

                memory.push(*n);
                
                if cdebug {
                    println!("LEN(a) : {} ",memory.len());
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
                println!("Access: {}  {}  {} ->  {:?}  <- {} Replaced:{} [Hits:{} Misses:{}]", n, hfunc(idx), leftStr, &memory[0..], riteStr, vfunc(victim), hits, miss);
            }

            addrIndex = addrIndex + 1;



        }

        println!("");
        println!("FINALSTATS hits {}   misses {}   hitrate  : {hitrate}" , hits, miss, hitrate =  format!("{:.*}",2,(100.0* hits as f32)/(hits+miss)as f32));
        println!("");
    }
}
