//done
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

// fn roundup(size:f32) -> f32{
//         value = 1.0;
//         while value < size{
//             value = value * 2.0;
//         }
//         value
// }

struct OS {
    page_size: i32,
    phys_pages: i32,
    phys_mem: i32,
    va_pages: i32,
    va_size: i32,
    pte_size: i32,
    page_bits: i32,

    used_pages: Vec<i32>,
    used_pages_count: i32,
    max_page_count: i32,

    memory: Vec<i32>,
    pdbr: HashMap<i32, i32>,

    pde_mask: u32,
    pde_shift: u32,

    pte_mask: u32,
    pte_shift: u32,

    vpn_mask: u32,
    vpn_shift: u32,

    offset_mask: u32,
}

impl OS {
    fn new() -> OS {
        OS {
            // phys memory 4k Byte (4096 Byte)
            page_size: 32,   // page size 32 Byte
            phys_pages: 128, // phys memory devide total 128 pages
            //phys_mem:self.page_size * self.phys_pages,
            phys_mem: 4096,
            va_pages: 1024,
            //va_size:self.page_size * self.va_pages,
            va_size: 32768,
            pte_size: 1,
            page_bits: 5,

            // tracks
            used_pages_count: 0,
            max_page_count: 128,
            used_pages: vec![0; 128],
            memory: vec![0; 4096],
            pdbr: HashMap::new(),

            //     15       11        5         1
            // 0111 1100 0000 0000  (0x7c00)
            pde_mask: 0x7c00,
            pde_shift: 10,
            // 0000 0011 1110 0000  (0x03e0)
            pte_mask: 0x03e0,
            pte_shift: 5,

            vpn_mask: 0x7fe0,
            vpn_shift: 5,

            offset_mask: 0x1f,
        }
    }

    fn find_free(&mut self) -> i32 {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
        //if self.used_pages_count < self.max_page_count {
        let rand_x: f32 = rng.gen();
        let mut look = (rand_x * self.max_page_count as f32) as usize;
        while self.used_pages[look] == 1 {
            let rand_y: f32 = rng.gen();
            look = (rand_y * self.max_page_count as f32) as usize;
        }
        self.used_pages_count = self.used_pages_count + 1;
        self.used_pages[look] = 1;
        look as i32
        //}
    }

    fn init_page_dir(&mut self, which_page: i32) {
        let which_byte = which_page << self.page_bits;
        for i in which_byte as usize..which_byte as usize + self.page_size as usize {
            self.memory[i] = 0x7fi32; // 0111 1111 (0x7f) ???
        }
    }

    fn init_page_table_page(&mut self, which_page: i32) {
        self.init_page_dir(which_page);
    }

    fn get_page_table_entry(
        &self,
        virtual_addr: u32,
        _pte_page: i32,
        print_stuff: bool,
    ) -> (i32, i32, usize) {
        let pte_bits = (virtual_addr & self.pte_mask) >> self.pte_shift;
        let pte_addr = ((_pte_page << self.page_bits) | pte_bits as i32) as usize;
        let pte = self.memory[pte_addr];
        let valid = (pte & 0x80) >> 7;
        let pfn = pte & 0x7f;
        if print_stuff == true {
            println!(
                "    --> pte index:0x{:x}  pte contents:(valid {}, pfn 0x{:x})",
                pte_bits, valid, pfn
            );
        }
        return (valid, pfn, pte_addr);
    }
    fn get_page_dir_entry(
        &self,
        pid: i32,
        virtual_addr: u32,
        print_stuff: bool,
    ) -> (i32, i32, usize) {
        let page_dir = self.pdbr[&pid];
        let pde_bits = (virtual_addr & self.pde_mask) >> self.pde_shift;
        let pde_addr = (page_dir << self.page_bits | pde_bits as i32) as usize;
        let pde = self.memory[pde_addr];
        let valid = (pde & 0x80) >> 7;
        let pt_ptr = pde & 0x7f;

        if print_stuff == true {
            println!(
                "  --> pde index:0x{:x}  pde contents:(valid {}, pfn 0x{:x})",
                pde_bits, valid, pt_ptr
            );
        }
        return (valid, pt_ptr, pde_addr);
    }

    fn set_page_table_entry(&mut self, pte_addr: usize, physical_page: i32) {
        self.memory[pte_addr] = 0x80 | physical_page;
    }

    fn set_page_dir_entry(&mut self, pde_addr: usize, physical_page: i32) {
        self.memory[pde_addr] = 0x80 | physical_page;
    }

    fn dump_page(&mut self, which_page: usize) {
        let i = which_page;
        for j in 0..self.page_size {
            print!(
                "{}",
                self.memory[(i * self.page_size as usize) + j as usize]
            );
        }
        println!("");
    }

    fn memory_dump(&self) {
        for i in 0..(self.phys_mem / self.page_size) {
            print!("page  {}: ", i);
            for j in 0..self.page_size {
                print!(
                    "{:x} ",
                    self.memory[(i as usize * self.page_size as usize) + j as usize]
                );
            }
            println!("");
        }
    }

    fn alloc_virtual_page(&mut self, pid: i32, virtual_page: u32, physical_page: i32) {
        let virtual_addr = virtual_page << self.page_bits;
        let (valid, pt_ptr, pde_addr) = self.get_page_dir_entry(pid, virtual_addr, false);
        let mut _pte_page = 0;
        if valid == 0 {
            assert_eq!(pt_ptr, 127);
            _pte_page = self.find_free();
            self.set_page_dir_entry(pde_addr, _pte_page);
            self.init_page_table_page(_pte_page);
        } else {
            _pte_page = pt_ptr;
        }
        let (valid, pfn, pte_addr) = self.get_page_table_entry(virtual_addr, _pte_page, false);
        assert_eq!(valid, 0);
        assert_eq!(pfn, 127);
        self.set_page_table_entry(pte_addr, physical_page);
    }

    fn translate(&mut self, pid: i32, virtual_addr: u32) -> i32 {
        let (valid, pt_ptr, _pde_addr) = self.get_page_dir_entry(pid, virtual_addr, true);
        if valid == 1 {
            let _pte_page = pt_ptr;
            let (valid, pfn, _pte_addr) = self.get_page_table_entry(virtual_addr, _pte_page, true);
            if valid == 1 {
                let offset = (virtual_addr & self.offset_mask) as i32;
                let paddr = (pfn << self.page_bits) | offset;
                return paddr;
            } else {
                return -2;
            }
        }
        return -1;
    }

    fn fill_page(&mut self, which_page: usize) {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
        for j in 0..self.page_size {
            let rand_x: f32 = rng.gen();
            self.memory[(which_page * self.page_size as usize) + j as usize] =
                (rand_x * 31.0f32) as i32
        }
    }

    fn proc_alloc(&mut self, pid: i32, num_pages: i32) -> Vec<i32> {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

        let page_dir = self.find_free();
        self.pdbr.insert(pid, page_dir);
        self.init_page_dir(page_dir);

        let mut used: HashMap<i32, i32> = HashMap::new();

        for vp in 0..self.va_pages {
            used.insert(vp, 0);
        }

        let mut allocated_vps: Vec<i32> = Vec::new();
        for mut _vp in 0..num_pages {
            let rand_x: f32 = rng.gen();
            _vp = (rand_x * self.va_pages as f32) as i32;
            while used[&_vp] == 1 {
                let rand_y: f32 = rng.gen();
                _vp = (rand_y * self.va_pages as f32) as i32;
            }
            assert_eq!(used[&_vp], 0);
            used.insert(_vp, 1);
            allocated_vps.push(_vp);
            let pp = self.find_free();

            self.alloc_virtual_page(pid, _vp as u32, pp);
            self.fill_page(pp as usize);
        }
        return allocated_vps;
    }

    fn get_pdbr(&mut self, pid: i32) -> i32 {
        return self.pdbr[&pid];
    }

    fn get_value(&mut self, addr: usize) -> i32 {
        return self.memory[addr];
    }
}

struct PMTOption {
    seed: u64,
    allocated: i32,
    num: i32,
    solve: bool,
}

impl PMTOption {
    fn new() -> PMTOption {
        PMTOption {
            seed: 0,
            allocated: 64,
            num: 10,
            solve: false,
        }
    }
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut pmt_op = PMTOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" => {
                pmt_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-a" => {
                pmt_op.allocated = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-n" => {
                pmt_op.num = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-c" => {
                pmt_op.solve = true;
                i = i + 1;
            }
            _ => {
                println!("pmt_op_parse match wrong!!");
                return;
            }
        }
    }
    execute_pmt_op(pmt_op);
}

fn execute_pmt_op(options: PMTOption) {
    println!("ARG seed {}", options.seed);
    println!("ARG allocated {}", options.allocated);
    println!("ARG num {}", options.num);
    println!("");

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(options.seed);

    let mut os = OS::new();
    let used = os.proc_alloc(1, options.allocated);

    os.memory_dump();
    let pdbr = os.get_pdbr(1);

    println!(
        "\nPDBR: {} (decimal) [This means the page directory is held in this page]\n",
        pdbr
    );

    for i in 0..options.num {
        let rand_x: f32 = rng.gen();
        let mut _vaddr = 0;
        if (rand_x * 100.0f32) > 50.0f32 || i > used.len() as i32 {
            let rand_y: f32 = rng.gen();
            _vaddr = (rand_y * 1024f32 * 32f32) as i32;
        } else {
            //let tmp = used[i as usize];
            let rand_z: f32 = rng.gen();
            _vaddr = (used[i as usize] << 5) | (rand_z * 32f32) as i32;
        }

        if options.solve == true {
            println!("Virtual Address {:x}:", _vaddr);
            let r = os.translate(1, _vaddr as u32);
            if r > -1 {
                println!(
                    "      --> Translates to Physical Address 0x{:x} --> Value: {:x}",
                    r,
                    &os.get_value(r as usize)
                );
            } else if r == -1 {
                println!("      --> Fault (page directory entry not valid)");
            } else {
                println!("      --> Fault (page table entry not valid)");
            }
        } else {
            println!("Virtual Address %{:x}: Translates To What Physical Address (And Fetches what Value)? Or Fault?" ,_vaddr);
        }
    }
}
