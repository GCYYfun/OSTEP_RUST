//done 
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

// fn roundup(size:f32) -> f32{
//         value = 1.0;
//         while value < size{
//             value = value * 2.0;
//         }
//         value
// }

struct OS {
    pageSize:i32,
    physPages:i32,
    physMem:i32,
    vaPages:i32,
    vaSize:i32,
    pteSize:i32,
    pageBits:i32,

    usedPages:Vec<i32>,
    usedPagesCount:i32,
    maxPageCount:i32,

    memory:Vec<i32>,
    pdbr:HashMap<i32,i32>,

    PDE_MASK:u32,
    PDE_SHIFT:u32,

    PTE_MASK:u32,
    PTE_SHIFT:u32,

    VPN_MASK:u32,
    VPN_SHIFT:u32,

    OFFSET_MASK:u32,
}

impl OS {
    fn new()->OS {
        OS {                           // phys memory 4k Byte (4096 Byte)
            pageSize:32,    // page size 32 Byte
            physPages:128, // phys memory devide total 128 pages
            //physMem:self.pageSize * self.physPages,
            physMem:4096,
            vaPages:1024,
            //vaSize:self.pageSize * self.vaPages,
            vaSize:32768,
            pteSize:1,
            pageBits:5,

            // tracks
            
            usedPagesCount:0,
            maxPageCount:128,
            usedPages:vec![0;128],
            memory:vec![0;4096],
            pdbr:HashMap::new(),

            //     15       11        5         1
            // 0111 1100 0000 0000  (0x7c00)
            PDE_MASK:0x7c00,
            PDE_SHIFT:10,
            // 0000 0011 1110 0000  (0x03e0)
            PTE_MASK:0x03e0,
            PTE_SHIFT:5,

            VPN_MASK:0x7fe0,
            VPN_SHIFT:5,

            OFFSET_MASK:0x1f,

        }
    }

    fn find_free(&mut self) -> i32{
        let seed_u8 = 0;
        let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

        let mut rng = SmallRng::from_seed(seed);
        //if self.usedPagesCount < self.maxPageCount {
            let rand_x:f32 = rng.gen();
            let mut look = (rand_x * self.maxPageCount as f32) as usize;
            while self.usedPages[look] ==1 {
                let rand_y:f32 = rng.gen();
                look = (rand_y * self.maxPageCount as f32) as usize;
            }
            self.usedPagesCount = self.usedPagesCount +1;
            self.usedPages[look] = 1;
            look as i32
        //}
        
    }

    fn init_page_dir(&mut self,whichPage:i32) {
        let whichByte = whichPage << self.pageBits;
        for i  in whichByte as usize..whichByte as usize+self.pageSize as usize {
            self.memory[i] = 0x7fi32;  // 0111 1111 (0x7f) ???
        }
    }

    fn init_page_table_page(&mut self,whichPage:i32) {
        self.init_page_dir(whichPage);
    }

    fn get_page_table_entry(&self,virtualAddr:u32,ptePage:i32,printStuff:bool) ->(i32,i32,usize){
        let pteBits = (virtualAddr & self.PTE_MASK) >> self.PTE_SHIFT;
        let pteAddr = ((ptePage << self.pageBits) | pteBits as i32) as usize;
        let pte     = self.memory[pteAddr];
        let valid   = (pte & 0x80) >> 7;
        let pfn     = (pte & 0x7f);
        if printStuff == true{
            println!("    --> pte index:0x{:x}  pte contents:(valid {}, pfn 0x{:x})" ,pteBits, valid, pfn);
        }
        return (valid, pfn, pteAddr);
    }
    fn get_page_dir_entry(&self, pid:i32, virtualAddr:u32, printStuff:bool) ->(i32,i32,usize){
        let pageDir = self.pdbr[&pid];
        let pdeBits = (virtualAddr & self.PDE_MASK) >> self.PDE_SHIFT;
        let pdeAddr = (pageDir << self.pageBits | pdeBits as i32) as usize;
        let pde = self.memory[pdeAddr];
        let valid = (pde & 0x80) >> 7;
        let ptPtr = (pde & 0x7f);

        if printStuff == true {
            println!("  --> pde index:0x{:x}  pde contents:(valid {}, pfn 0x{:x})" ,pdeBits, valid, ptPtr);
        }
        return (valid, ptPtr, pdeAddr);
    }

    fn set_page_table_entry(&mut self, pteAddr:usize, physicalPage:i32){
        self.memory[pteAddr] = 0x80 | physicalPage;
    }

    fn set_page_dir_entry(&mut self, pdeAddr:usize, physicalPage:i32){
        self.memory[pdeAddr] = 0x80 | physicalPage;
    }


    fn  dump_page(&mut self, whichPage:usize){
        let i = whichPage;
            for j in 0..self.pageSize{
                print!("{}", self.memory[(i * self.pageSize as usize) + j as usize]);
            }
        println!("");
    }

    fn memory_dump(&self){
        for i in 0..(self.physMem / self.pageSize){
            print!("page  {}: " ,i);
            for j in (0..self.pageSize){
                print!("{:x} " , self.memory[(i as usize * self.pageSize as usize) + j as usize]);
            }
            println!("");
        }
    }

    fn alloc_virtual_page(&mut self,pid:i32,virtualPage:u32,physicalPage:i32){
        let virtualAddr = virtualPage << self.pageBits;
        let (valid,ptPtr,pdeAddr) = self.get_page_dir_entry(pid, virtualAddr, false);
        let mut ptePage = 0;
        if valid == 0{
            assert_eq!(ptPtr,127);
            ptePage = self.find_free();
            self.set_page_dir_entry(pdeAddr, ptePage);
            self.init_page_table_page(ptePage);
        }else {
            ptePage = ptPtr;
        }
        let (valid, pfn, pteAddr) = self.get_page_table_entry(virtualAddr, ptePage, false);
        assert_eq!(valid , 0);
        assert_eq!(pfn , 127);
        self.set_page_table_entry(pteAddr, physicalPage);
    }

    fn translate(&mut self,pid:i32,virtualAddr:u32) ->i32 {
        let (valid, ptPtr, pdeAddr) = self.get_page_dir_entry(pid, virtualAddr, true);
        if valid == 1{
            let ptePage = ptPtr;
            let (valid, pfn, pteAddr) = self.get_page_table_entry(virtualAddr, ptePage, true);
            if valid == 1{
                let offset = (virtualAddr & self.OFFSET_MASK) as i32;
                let paddr  = (pfn << self.pageBits) | offset;
                return paddr;
            }else{
                return -2;
            }
        }
        return -1
    }

    fn fill_page(&mut self, whichPage:usize){
        let seed_u8 = 0;
        let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

        let mut rng = SmallRng::from_seed(seed);
        for j in 0..self.pageSize{
            let rand_x:f32 = rng.gen();
            self.memory[(whichPage * self.pageSize as usize) + j as usize] = (rand_x * 31.0f32) as i32
        }
    }

    fn proc_alloc(&mut self,pid:i32,numPages:i32) ->Vec<i32>{
        let seed_u8 = 0;
        let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

        let mut rng = SmallRng::from_seed(seed);

        let pageDir = self.find_free();
        self.pdbr.insert(pid,pageDir);
        self.init_page_dir(pageDir);

        let mut used:HashMap<i32,i32> = HashMap::new();

        for vp in 0..self.vaPages {
            used.insert(vp, 0);
        }

        let mut allocatedVPs:Vec<i32> = Vec::new();
        for mut vp in  0..numPages{
            let rand_x:f32 = rng.gen();
            vp = (rand_x * self.vaPages as f32) as i32;
            while used[&vp] ==1 {
                let rand_y:f32 = rng.gen();
                vp = (rand_y * self.vaPages as f32) as i32;
            }
            assert_eq!(used[&vp],0);
            used.insert(vp,1);
            allocatedVPs.push(vp);
            let pp = self.find_free();

            self.alloc_virtual_page(pid, vp as u32, pp);
            self.fill_page(pp as usize);
        }
        return allocatedVPs;
    }


    fn get_PDBR(&mut self, pid:i32)->i32{
        return self.pdbr[&pid];
    }

    fn get_value(&mut self, addr:usize)->i32{
        return self.memory[addr];
    }
}


struct PMTOption {
    seed:i32,
    allocated:i32,
    num:i32,
    solve:bool,
}

impl PMTOption {
    fn new() -> PMTOption {
        PMTOption {
            seed:0,
            allocated:64,
            num:10,
            solve:false,
        }
    }
}

pub fn pmt_op_parse(op_vec:Vec<&str>) {
        let mut pmt_op = PMTOption::new();
        let mut i =1;
        while i<op_vec.len() {
            match op_vec[i] {
                "-s" =>{pmt_op.seed = op_vec[i+1].parse().unwrap();i = i+2;},
                "-a" =>{pmt_op.allocated = op_vec[i+1].parse().unwrap();i = i+2;},
                "-n" =>{pmt_op.num = op_vec[i+1].parse().unwrap();i=i+2;},
                "-c" =>{pmt_op.solve = true;i=i+1;},
                _ => println!("pmt_op_parse match wrong!!"),
            }
        }
        execute_pmt_op(pmt_op);
}

fn execute_pmt_op(options:PMTOption) {
    println! ("ARG seed {}", options.seed);
    println! ("ARG allocated {}",  options.allocated);
    println! ("ARG num {}",  options.num);
    println! ("");

    let seed_u8 = options.seed as u8;
    let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

    let mut rng = SmallRng::from_seed(seed);

    let mut os = OS::new();
    let used = os.proc_alloc(1, options.allocated);
    
    os.memory_dump();
    let pdbr = os.get_PDBR(1);

    println!("\nPDBR: {} (decimal) [This means the page directory is held in this page]\n", pdbr);
    
    for i in 0..options.num {
        let rand_x:f32 = rng.gen();
        let mut  vaddr = 0;
        if (rand_x * 100.0f32)> 50.0f32 || i >used.len() as i32 {
            let rand_y:f32 = rng.gen();
            vaddr = (rand_y*1024f32*32f32)as i32;
        }else {
            //let tmp = used[i as usize];
            let rand_z:f32 = rng.gen();
            vaddr = (used[i as usize] << 5) | (rand_z* 32f32) as i32;
        }

        if options.solve == true {
            println!("Virtual Address {:x}:" ,vaddr);
            let r = os.translate(1, vaddr as u32);
            if r > -1 {
                println!("      --> Translates to Physical Address 0x{:x} --> Value: {:x}" ,r, &os.get_value(r as usize));
            }else if r ==-1 {
                 println!("      --> Fault (page directory entry not valid)");
            }else {
                println!("      --> Fault (page table entry not valid)");
            }
        }else {
            println!("Virtual Address %{:x}: Translates To What Physical Address (And Fetches what Value)? Or Fault?" ,vaddr);
        }
    }
}