//  done

use rand::{Rng, SeedableRng};

const HELP: &str = include_str!("help.txt");

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

struct SegmentationOption {
    seed: u64,
    addresses: String,
    asize: String,
    psize: String,
    num: i32,
    base0: String,
    len0: String,
    base1: String,
    len1: String,
    solve: bool,
}

impl SegmentationOption {
    fn new() -> SegmentationOption {
        SegmentationOption {
            seed: 0,
            addresses: String::from("-1"),
            asize: String::from("1k"),
            psize: String::from("16k"),
            num: 5,
            base0: String::from("-1"),
            len0: String::from("-1"),
            base1: String::from("-1"),
            len1: String::from("-1"),
            solve: false,
        }
    }
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut segm_op = SegmentationOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" => {
                segm_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-A" => {
                segm_op.addresses = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-a" => {
                segm_op.asize = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-p" => {
                segm_op.psize = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-n" => {
                segm_op.num = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-b" => {
                segm_op.base0 = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-l" => {
                segm_op.len0 = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-B" => {
                segm_op.base1 = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-L" => {
                segm_op.len1 = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-c" => {
                segm_op.solve = true;
                i = i + 1;
            }
            _ => {
                println!("segmentation_op_parse match wrong!!");
                return;
            }
        }
    }
    execute_segmentation_op(segm_op);
}

fn execute_segmentation_op(options: SegmentationOption) {
    println!("");
    println!("ARG seed : {} ", options.seed);
    println!("ARG address space size : {} ", options.asize);
    println!("ARG phys mem size : {} ", options.psize);
    println!("");

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(options.seed);

    let asize = convert(options.asize);
    let psize = convert(options.psize);
    let addresses = options.addresses;

    if psize <= 1 {
        println!("Error: must specify a non-zero physical memory size.");
        return;
    }

    if asize == 0 {
        println!("Error: must specify a non-zero address-space size.");
        return;
    }

    if psize <= asize {
        println!("Error: physical memory size must be GREATER than address space size (for this simulation)");
        return;
    }

    let mut len0 = convert(options.len0);
    let mut len1 = convert(options.len1);
    let mut base0 = convert(options.base0);
    let mut base1 = convert(options.base1);

    if len0 == -1 {
        let rand_x: f32 = rng.gen();
        len0 = ((asize as f32 / 4.0) + (asize as f32 / 4.0 * rand_x)) as i32;
    }

    if len1 == -1 {
        let rand_x: f32 = rng.gen();
        len1 = ((asize as f32 / 4.0) + (asize as f32 / 4.0 * rand_x)) as i32;
    }

    if base0 == -1 {
        loop {
            let rand_x: f32 = rng.gen();
            base0 = (psize as f32 * rand_x) as i32;
            if (base0 + len0) < psize {
                break;
            }
        }
    }

    if base1 == -1 {
        loop {
            let rand_x: f32 = rng.gen();
            base1 = (psize as f32 * rand_x) as i32;
            if (base1 + len1) < psize {
                if (base1 > (base0 + len0)) || ((base1 + len1) < base0) {
                    break;
                }
            }
        }
    } else {
        base1 = base1 - len1;
    }

    if len0 > asize / 2 || len1 > asize / 2 {
        // not very good idea
        println!("Error: length register is too large for this address space");
        return;
    }

    println!("Segment register information:");
    println!("");
    println!(
        "  Segment 0 base  (grows positive) : 0x{:x} (decimal {})",
        base0, base0
    );
    println!("  Segment 0 limit                  : {}", len0);
    println!("");
    println!(
        "  Segment 1 base  (grows negative) : 0x{:x} (decimal {})",
        base1 + len1,
        base1 + len1
    );
    println!("  Segment 1 limit                  : {}", len1);
    println!("");
    let nbase1 = base1 + len1;

    if (len0 + base0) > (base1) && (base1 > base0) {
        println!("Error: segments overlap in physical memory");
        return;
    }

    let mut addr_list: Vec<i32> = Vec::new();

    if addresses == "-1" {
        for _i in 0..options.num {
            let rand_x: f32 = rng.gen();
            let n = (asize as f32 * rand_x) as i32;
            addr_list.push(n);
        }
    } else {
        let vec: Vec<&str> = addresses.split(",").collect();
        for t in vec {
            addr_list.push(t.to_string().parse().unwrap());
        }
    }

    println!("Virtual Address Trace");
    let mut i = 0;
    for v_str in &addr_list {
        let vaddr: i32 = v_str.to_string().parse().unwrap();
        if vaddr < 0 || vaddr >= asize {
            println!(
                "Error: virtual address {} cannot be generated in an address space of size {}",
                vaddr, asize
            );
            return;
        }

        if options.solve == false {
            println!(
                "  VA   {} : 0x{:x} (decimal:     {}) --> PA or segmentation violation?",
                i, vaddr, vaddr
            );
        } else {
            let mut _paddr = 0;
            if vaddr >= (asize / 2) {
                _paddr = nbase1 + (vaddr - asize);
                if _paddr < base1 {
                    println!(
                        "  VA {}: 0x{:x} (decimal:  {}) --> SEGMENTATION VIOLATION (SEG1)",
                        i, vaddr, vaddr
                    );
                } else {
                    println!(
                        "  VA {}: 0x{:x} (decimal:  {}) --> VALID in SEG1: 0x{:x} (decimal:  {})",
                        i, vaddr, vaddr, _paddr, _paddr
                    );
                }
            } else {
                if vaddr >= len0 {
                    println!(
                        "  VA  {}: 0x{:x} (decimal: {}) --> SEGMENTATION VIOLATION (SEG0)",
                        i, vaddr, vaddr
                    );
                } else {
                    _paddr = vaddr + base0;
                    println!(
                        "  VA  {}: 0x{:x} (decimal:  {}) --> VALID in SEG0: 0x{:x} (decimal:  {})",
                        i, vaddr, vaddr, _paddr, _paddr
                    );
                }
            }
        }
        i = i + 1;
    }

    println!("");

    if options.solve == false {
        println!(
            "For each virtual address, either write down the physical address it translates to"
        );
        println!("OR write down that it is an out-of-bounds address (a segmentation violation). Forthis problem, you should assume a simple address space with two segments: the top");
        println!(
            "this problem, you should assume a simple address space with two segments: the top"
        );
        println!(
            "bit of the virtual address can thus be used to check whether the virtual address"
        );
        println!(
            "is in segment 0 (topbit=0) or segment 1 (topbit=1). Note that the base/limit pairs"
        );
        println!(
            "given to you grow in different directions, depending on the segment, i.e., segment 0"
        );
        println!("grows in the positive direction, whereas segment 1 in the negative. ");
        println!("");
    }
}
