// done

use rand::{Rng, SeedableRng};

const HELP: &str = include_str!("help.txt");

fn mustbepowerof2(bits: u32, size: i32, msg: String) {
    if (2 as i32).pow(bits) != size {
        println!("Error in argument : {} ", msg);
        return;
    }
}

fn mustbemultipleof(bignum: i32, num: i32, msg: String) {
    if ((bignum / num) as i32) != (bignum as f32 / num as f32) as i32 {
        println!("Error in argument : {} ", msg);
        return;
    }
}

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

struct PLTOption {
    seed: u64,
    asize: String,
    addresses: String,
    psize: String,
    pagesize: String,
    num: i32,
    used: i32,
    verbose: bool,
    solve: bool,
}

impl PLTOption {
    fn new() -> PLTOption {
        PLTOption {
            seed: 0,
            asize: String::from("16k"),
            addresses: String::from("-1"),
            psize: String::from("64k"),
            pagesize: String::from("4k"),
            num: 5,
            used: 50,
            verbose: false,
            solve: false,
        }
    }
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut plt_op = PLTOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" => {
                plt_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-a" => {
                plt_op.asize = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-A" => {
                plt_op.addresses = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-p" => {
                plt_op.psize = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-P" => {
                plt_op.pagesize = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-n" => {
                plt_op.num = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-u" => {
                plt_op.used = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-v" => {
                plt_op.verbose = true;
                i = i + 1;
            }
            "-c" => {
                plt_op.solve = true;
                i = i + 1;
            }
            _ => {
                println!("plt_op_parse match wrong!!");
                return;
            }
        }
    }
    execute_plt_op(plt_op);
}

fn execute_plt_op(options: PLTOption) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(options.seed);
    println!("ARG seed : {}", options.seed);
    println!("ARG address space size : {}", options.asize);
    println!("ARG phys mem size : {}", options.psize);
    println!("ARG page size : {}", options.pagesize);
    println!("ARG verbose : {}", options.verbose);
    println!("ARG addresses : {}", options.addresses);
    println!("");

    let asize = convert(options.asize);
    let psize = convert(options.psize);
    let pagesize = convert(options.pagesize);
    let addresses = options.addresses;

    if psize <= 1 {
        println!("Error: must specify a non-zero physical memory size.");
        return;
    }

    if asize < 1 {
        println!("Error: must specify a non-zero address-space size.");
        return;
    }

    if psize <= asize {
        println! ("Error: physical memory size must be GREATER than address space size (for this simulation)");
        return;
    }

    if psize >= convert("1g".to_string()) || asize >= convert("1g".to_string()) {
        println!("Error: must use smaller sizes (less than 1 GB) for this simulation.");
        return;
    }

    mustbemultipleof(
        asize,
        pagesize,
        "address space must be a multiple of the pagesize".to_string(),
    );
    mustbemultipleof(
        psize,
        pagesize,
        "physical memory must be a multiple of the pagesize".to_string(),
    );

    let pages = (psize / pagesize) as usize;
    let mut used = vec![0; pages];
    let mut pt: Vec<i32> = Vec::new();

    let vpages = asize / pagesize;

    let vabits = ((asize as f32).ln() / 2.0f32.ln()) as u32;
    mustbepowerof2(
        vabits,
        asize,
        "address space must be a power of 2".to_string(),
    );
    let pagebits = ((pagesize as f32).ln() / 2.0f32.ln()) as u32;
    mustbepowerof2(
        pagebits,
        pagesize,
        "page size must be a power of 2".to_string(),
    );
    // let vpnbits = vabits - pagebits;
    let pagemask = (1 << pagebits) - 1;

    let vpnmask = 0xFFFFFFFFu32 & !pagemask;

    println!("");
    println!("The format of the page table is simple:");
    println!("The high-order (left-most) bit is the VALID bit.");
    println!("  If the bit is 1, the rest of the entry is the PFN.");
    println!("  If the bit is 0, the page is not valid.");
    println!("Use verbose mode (-v) if you want to println! the VPN # by");
    println!("each entry of the page table.");
    println!("");

    println!("Page Table (from entry 0 down to the max size)");
    for v in 0..vpages {
        loop {
            let rand_x: f32 = rng.gen();
            if (rand_x * 100.0f32) > (100.0f32 - options.used as f32) {
                let rand_y: f32 = rng.gen();
                let u = (pages as f32 * rand_y) as usize;
                if used[u] == 0 {
                    used[u] = 1;
                    if options.verbose == true {
                        print!("  [  {}  ]  ", v);
                    } else {
                        print!(" ");
                    }
                    println!(" 0x{:08x}", (0x80000000u32 | u as u32));
                    pt.insert(v as usize, u as i32);
                    break;
                } else {
                    if options.verbose == true {
                        print!("  [  {}  ]  ", v);
                    } else {
                        print!(" ");
                    }
                    println!(" 0x{:08x}", 0);
                    pt.insert(v as usize, -1);
                    break;
                }
            }
        }
    }
    println!("");

    let mut addr_list: Vec<u32> = Vec::new();
    if addresses == "-1" {
        //# need to generate addresses
        for _i in 0..options.num {
            let rand_x: f32 = rng.gen();
            let n = (asize as f32 * rand_x) as u32;
            addr_list.push(n);
        }
    } else {
        let addr_strlist: Vec<&str> = addresses.split(',').collect();
        for addr_str in addr_strlist {
            addr_list.push(addr_str.parse().unwrap());
        }
    }

    println!("Virtual Address Trace");
    for i in addr_list {
        let vaddr = i;
        if options.solve == false {
            println!(
                "  VA 0x{:08x} (decimal:    {}) --> PA or invalid address?",
                vaddr, vaddr
            );
        } else {
            let mut _paddr = 0;
            //# split vaddr into VPN | offset
            let vpn = (vaddr & vpnmask) >> pagebits;
            if pt[vpn as usize] < 0 {
                println!(
                    "  VA 0x{:08x} (decimal:     {}) --> Invalid (VPN {} not valid)",
                    vaddr, vaddr, vpn
                );
            } else {
                let pfn = pt[vpn as usize];
                let offset = vaddr & pagemask;
                _paddr = ((pfn as u32) << pagebits) | offset;
                println!(
                    "  VA 0x{:08x} (decimal:     {}) --> 0x{:08x} (decimal    {}) [VPN {}]",
                    vaddr, vaddr, _paddr, _paddr, vpn
                );
            }
        }
    }

    println!("");
    if options.solve == false {
        println!("For each virtual address, write down the physical address it translates to");
        println!("OR write down that it is an out-of-bounds address (e.g., segfault).");
        println!("");
    }
}
