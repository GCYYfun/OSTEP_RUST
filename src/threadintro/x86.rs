use std::collections::HashMap;
use rand::prelude::*;
//
// HELPER
//

fn dospace(howmuch:i32)
{
    for i in 0..howmuch {
        print!("{:width$}",width = 24);
    }
}

fn zassert(cond:bool,s:String,)
{
    if cond == false {
        println!("ABORT::{}",s);
        println!("exit()");
    }
}
#[derive(Debug)]
struct Cpu {
    //condition
    COND_GT:u8,
    COND_GTE:u8,
    COND_LT:u8,
    COND_LTE:u8,
    COND_EQ:u8,
    COND_NEQ:u8,
    //registers
    REG_ZERO:u8,
    REG_AX:u8,
    REG_BX:u8,
    REG_CX:u8,
    REG_DX:u8,
    REG_SP:u8,
    REG_BP:u8,
    // memory  unit : KB
    max_memory:i32,
    //which memory addrs and registers to trace
    memtrace:Vec<String>,
    regtrace:Vec<String>,
    cctrace:bool,
    compute:bool,
    verbose:bool,
    PC:i32,
    registers:HashMap<u8,i32>,
    conditions:HashMap<u8,bool>,
    labels:HashMap<i32,i32>,
    vars:HashMap<i32,i32>,
    memory:HashMap<i32,i32>,
    pmemory:HashMap<i32,i32>,
    condlist:Vec<u8>,
    regnums:Vec<u8>,
    regnames:HashMap<String,u8>,
    


}

impl Cpu{
    fn new(memory:i32,memtrace:Vec<&str>,regtrace:Vec<&str>,cctrace:bool,compte:bool,verbose:bool)->Cpu{
        Cpu {
               
                COND_GT:0,
                COND_GTE:1,
                COND_LT:2,
                COND_LTE:3,
                COND_EQ:4,
                COND_NEQ:5,
                
                REG_ZERO:0,
                REG_AX:1,
                REG_BX:2,
                REG_CX:3,
                REG_DX:4,
                REG_SP:5,
                REG_BP:6,
                
                max_memory:memory*1024,
                //which memory addrs and registers to trace
                memtrace:memtrace.iter().map(|s| s.to_string()).collect(),
                regtrace:regtrace.iter().map(|s| s.to_string()).collect(),
                cctrace:cctrace,
                compute:compte,
                verbose:verbose,

                PC:0,
                registers:HashMap::new(),
                conditions:HashMap::new(),
                labels:HashMap::new(),
                vars:HashMap::new(),
                memory:HashMap::new(),
                pmemory:HashMap::new(),
                condlist:Vec::new(),
                regnums:Vec::new(),
                regnames:HashMap::new(),
        }
    }

    fn init(&mut self) {
        self.condlist = [self.COND_GTE, self.COND_GT, self.COND_LTE, self.COND_LT, self.COND_NEQ, self.COND_EQ].to_vec();
        self.regnums = [self.REG_ZERO, self.REG_AX,  self.REG_BX,   self.REG_CX,  self.REG_DX,   self.REG_SP,  self.REG_BP].to_vec();
        self.regnames = [("zero".to_string(),self.REG_ZERO),("ax".to_string(),self.REG_AX),("bx".to_string(),self.REG_BX),("cx".to_string(),self.REG_CX),("dx".to_string(),self.REG_DX),("sp".to_string(),self.REG_SP),("bp".to_string(),self.REG_BP)].iter().cloned().collect();

        let mut templist:Vec<String> = Vec::new();
        for r in &self.regtrace {
            if self.regnames.contains_key(r) {
                templist.push(r.to_string());
            }else {
                println!("Register {} cannot be traced because it does not exist", r);
            }
        }
        self.regtrace = templist;

        self.init_memory();
        self.init_registers();
        self.init_condition_codes();
    }
    //
    // BEFORE MACHINE RUNS
    //
    fn init_memory(&mut self){
        for i in 0..self.max_memory {
            self.memory.entry(i).or_insert(0);
        }
    }
    fn init_registers(&mut self){
        for i in &self.regnums {
            self.registers.entry(*i).or_insert(0);
        }
    }
    fn init_condition_codes(&mut self){
        for c in &self.condlist {
            self.conditions.entry(*c).or_insert(false);
        }
    }

    fn dump_memory(&mut self){
        println!("MEMORY DUMP");
        for i in 0..self.max_memory {
            if !self.pmemory.contains_key(&i) && self.memory.contains_key(&i) && self.memory[&i] != 0 {
                println!(" m[{}] {}",i,self.memory[&i]);
            }
        }
    }

    // INFORMING ABOUT THE HARDWARE

    fn get_regnum(&mut self, name:String) -> u8{
        assert!(self.regnames.contains_key(&name));
        return self.regnames[&name];
    }

    fn get_regname(&mut self,num:u8) -> String{
        assert!(self.regnums.contains(&num));
        for rname in self.regnames.keys() {
            if self.regnames[rname] == num {
                 return rname.clone();
            }
        }
        return "".into();
    }

    fn get_regnums(&mut self) ->Vec<u8>{
        return self.regnums.clone();
    }

    fn get_condlist(&mut self) -> Vec<u8> {
        return self.condlist.clone();
    }

    fn get_reg(&mut self,reg:u8) ->i32{
        assert!(self.regnums.contains(&reg));
        return self.registers[&reg];
    }

    fn get_cond(&mut self,cond:u8) ->bool{
        assert!(self.condlist.contains(&cond));
        return self.conditions[&cond];
    }

    fn ge_pc(&self) -> i32{
        return self.PC;
    }

    fn set_reg(&mut self,reg:u8,value:i32) {
        assert!(self.regnums.contains(&reg));
        self.registers.insert(reg,value);
    }

    fn set_cond(&mut self,cond:u8,value:bool) {
        assert!(self.condlist.contains(&cond));
        self.conditions.insert(cond,value);
    }

    fn set_pc(&mut self,pc:i32) {
        self.PC = pc;
    }

    //
    //  INSTRUCTIONS
    // 

    fn halt(&self) ->i32 {
        return -1;
    }

    fn iyield(&self) ->i32 {
        return -2;
    }

    fn nop(&self) -> i32 {
        return 0;
    }

    fn rdump(&self) {
        print!("REGISTERS::");
        print!("ax: {} ",self.registers[&self.REG_AX]);
        print!("bx: {} ",self.registers[&self.REG_BX]);
        print!("cx: {} ",self.registers[&self.REG_CX]);
        print!("dx: {} ",self.registers[&self.REG_DX]);
    }

    fn mdump(&self,index:i32) {
        println!("m[{}] {}",index,self.memory[&index]);
    }

    fn move_i_to_r(&mut self,src:i32,dst:u8) -> i32 {
        self.registers.insert(dst,src);
        return 0;
    }

    //  memory: value, register, register

    fn move_i_to_m(&mut self,src:i32,value:i32,reg1:u8,reg2:u8) -> i32{
        let tmp = value + self.registers[&reg1] + self.registers[&reg2];
        self.memory.insert(tmp,src);
        return 0;
    }

    fn move_m_to_r(&mut self,value:i32,reg1:u8,reg2:u8,dst:u8) {
        let tmp = value + self.registers[&reg1] + self.registers[&reg2];
        self.registers.insert(dst,self.memory[&tmp]);
    }

    fn move_r_to_m(&mut self,src:u8,value:i32,reg1:u8,reg2:u8) -> i32{
        let tmp = value + self.registers[&reg1] + self.registers[&reg2];
        self.memory.insert(tmp,self.registers[&src]);
        return 0;
    }

    fn move_r_to_r(&mut self, src:u8, dst:u8) -> i32{
        self.registers.insert(dst,self.registers[&src]);
        return 0;
    }

    fn add_i_to_r(&mut self, src:i32, dst:u8) -> i32{
        self.registers.insert(dst,self.registers[&dst]+src);
        return 0;
    }

    fn add_r_to_r(&mut self, src:u8, dst:u8) -> i32{
        self.registers.insert(dst,self.registers[&dst]+self.registers[&src]);
        return 0;
    }

    fn sub_i_to_r(&mut self, src:i32, dst:u8) -> i32{
        self.registers.insert(dst,self.registers[&dst]-src);
        return 0;
    }

    fn sub_r_to_r(&mut self, src:u8, dst:u8) -> i32{
        self.registers.insert(dst,self.registers[&dst]-self.registers[&src]);
        return 0;
    }

    //
    // SUPPORT FOR LOCKS
    //

    fn atomic_exchange(&mut self,src:u8,value:i32,reg1:u8,reg2:u8) -> i32 {
        let tmp = value + self.registers[&reg1] + self.registers[&reg2];
        let old = self.memory[&tmp];
        self.memory.insert(tmp,self.registers[&src]);
        self.registers.insert(src,old);
        return 0;
    }

    fn fetchadd(&mut self,src:u8,value:i32,reg1:u8,reg2:u8) -> i32 {
        let tmp = value + self.registers[&reg1] + self.registers[&reg2];
        let old = self.memory[&tmp];
        self.memory.insert(tmp,self.memory[&tmp]+self.registers[&src]);
        self.registers.insert(src,old);
        return 0;
    }

    //
    //  TEST for conditions
    //

    fn test_all(&mut self,src:u8,dst:u8) -> i32 {
        self.init_condition_codes();
        if dst > src {
            self.conditions.insert(self.COND_GT,true);
        }
        if dst >= src {
            self.conditions.insert(self.COND_GTE,true);
        }
        if dst < src {
            self.conditions.insert(self.COND_LT, true);
        }
        if dst <= src {
            self.conditions.insert(self.COND_LTE,true);
        }
        if dst == src {
            self.conditions.insert(self.COND_EQ,true);
        }
        if dst != src {
            self.conditions.insert(self.COND_NEQ,true);
        }
        return 0

    }

    fn test_i_r(&mut self,src:u8,dst:u8) -> i32 {
        self.init_condition_codes();
        return self.test_all(src,self.registers[&dst] as u8);
    }

    fn test_r_i(&mut self,src:u8,dst:u8) -> i32 {
        self.init_condition_codes();
        return self.test_all(self.registers[&src] as u8,dst);
    }

    fn test_r_r(&mut self,src:u8,dst:u8) -> i32 {
        self.init_condition_codes();
        return self.test_all(self.registers[&src] as u8,self.registers[&dst] as u8);
    }

    //
    //  JUMPS
    //

    fn jump(&mut self,targ:i32) -> i32{
        self.PC = targ;
        return 0;
    }

    fn jump_notequal(&mut self,targ:i32) -> i32{
        if self.conditions[&self.COND_NEQ] == true {
            self.PC = targ;
        }
        return 0;
    }

    fn jump_equal(&mut self,targ:i32) -> i32{
        if self.conditions[&self.COND_EQ] == true {
            self.PC = targ;
        }
        return 0;
    }

    fn jump_lessthan(&mut self,targ:i32) -> i32{
        if self.conditions[&self.COND_LT] == true {
            self.PC = targ;
        }
        return 0;
    }

    fn jump_lessthanorequal(&mut self,targ:i32) -> i32{
        if self.conditions[&self.COND_LTE] == true {
            self.PC = targ;
        }
        return 0;
    }

    fn jump_greaterthan(&mut self,targ:i32) -> i32{
        if self.conditions[&self.COND_GT] == true {
            self.PC = targ;
        }
        return 0;
    }

    fn jump_greaterthanorequal(&mut self,targ:i32) -> i32{
        if self.conditions[&self.COND_GTE] == true {
            self.PC = targ;
        }
        return 0;
    }

    //
    //  CALL and RETURN
    //

    fn call(&mut self,targ:i32) {
        // let r = self.registers.entry(self.REG_SP);
        // r -= 4;
        // let m = self.memory.entry(self.registers[&self.REG_SP]);
        // m = self.PC;
        self.registers.insert(self.REG_SP,self.registers[&self.REG_SP] - 4);
        self.memory.insert(self.registers[&self.REG_SP],self.PC);
        self.PC = targ;
    }

    fn ret(&mut self) {
        self.PC = self.memory[&self.registers[&self.REG_SP]];
        self.registers.insert(self.REG_SP,self.registers[&self.REG_SP] + 4);
    }

    //
    //  STACK and related
    //

    fn push_r(&mut self,reg:u8) -> i32{
        self.registers.insert(self.REG_SP,self.registers[&self.REG_SP] - 4);
        self.memory.insert(self.registers[&self.REG_SP],self.registers[&reg]);
        return 0;
    }

    fn push_m(&mut self,value:i32,reg1:u8,reg2:u8) -> i32 {
        self.registers.insert(self.REG_SP,self.registers[&self.REG_SP] - 4);
        let tmp = value + self.registers[&reg1] + self.registers[&reg2];

        self.memory.insert(self.registers[&self.REG_SP],tmp);
        return 0;
    }

    fn pop(&mut self) {
        self.registers.insert(self.REG_SP,self.registers[&self.REG_SP] + 4);
    }

    fn pop_r(&mut self,dst:u8) {
        self.registers.insert(dst,self.registers[&self.REG_SP]);
        self.registers.insert(self.REG_SP,self.registers[&self.REG_SP] + 4);
    }

    //
    // HELPER func for getarg
    //

    fn register_translate(&mut self,r:String) ->u8{
        if self.regnames.contains_key(&r) {
            return self.regnames[&r];
        }
        return 0;
    }

    // HELPER in parsing mov (quite primitive) and other ops
    // returns: (value, type)
    // where type is (TYPE_REGISTER, TYPE_IMMEDIATE, TYPE_MEMORY)
    // 
    // FORMATS
    //    %ax           - register
    //    $10           - immediate
    //    10            - direct memory
    //    10(%ax)       - memory + reg indirect
    //    10(%ax,%bx)   - memory + 2 reg indirect
    //    10(%ax,%bx,4) - XXX (not handled)
    //

    fn getarg(&mut self,arg:String) {

    }

    //
    // LOAD a program into memory
    // make it ready to execute
    //

    fn load(&mut self,infile:String,loadaddr:i32) {
        
    }


}

struct Proclist{

}

struct Procsee {

}

#[derive(Debug)]
struct X86Option {
    seed:i32,
    numthreads:i32,
    progfile:String,
    intfreq:i32,
    intrand:bool,
    argv:String,
    loadaddr:i32,
    memsize:i32,
    memtrace:String,
    regtrace:String,
    cctrace:bool,
    printstats:bool,
    verbose:bool,
    solve:bool,
}

impl X86Option {
    fn new() -> X86Option {
        X86Option {
            seed:0,
            numthreads:2,
            progfile:String::from(""),
            intfreq:50,
            intrand:false,
            argv:String::from(""),
            loadaddr:1000,
            memsize:128,
            memtrace:String::from(""),
            regtrace:String::from(""),
            cctrace:false,
            printstats:false,
            verbose:false,
            solve:false,
        }
    }
}

pub fn x86_op_parse(op_vec:Vec<&str>) {
    let mut x86_op = X86Option::new();
    let mut i =1;
    while i<op_vec.len() {
        match op_vec[i] {
            "-s" =>{x86_op.seed = op_vec[i+1].parse().unwrap();i = i+2;},
            "-t" =>{x86_op.numthreads = op_vec[i+1].parse().unwrap();i = i+2;},
            "-p" =>{x86_op.progfile = op_vec[i+1].to_string();i=i+2;},
            "-i" =>{x86_op.intfreq = op_vec[i+1].parse().unwrap();i = i+2;},
            "-r" =>{x86_op.intrand = true;i=i+1;},
            "-a" =>{x86_op.argv = op_vec[i+1].to_string();i=i+2;},
            "-L" =>{x86_op.loadaddr = op_vec[i+1].parse().unwrap();i = i+2;},
            "-m" =>{x86_op.memsize = op_vec[i+1].parse().unwrap();i = i+2;},
            "-M" =>{x86_op.memtrace = op_vec[i+1].to_string();i=i+2;},
            "-R" =>{x86_op.regtrace = op_vec[i+1].to_string();i=i+2;},
            "-C" =>{x86_op.cctrace = true;i=i+1;},
            "-S" =>{x86_op.printstats = true;i=i+1;},
            "-v" =>{x86_op.verbose = true;i=i+1;},
            "-c" =>{x86_op.solve = true;i=i+1;},
            _ => println!("x86_op_parse match wrong!!"),
        }
    }
    execute_x86_op(x86_op);
}

fn execute_x86_op(options:X86Option) {

    println!("{:?}",options);

    let memsize    = options.memsize;
    let mut memtrace = vec![];
    if options.memtrace != "" {
        memtrace = options.memtrace.split(",").collect();
    }
    let mut regtrace = vec![];
    if options.regtrace != "" {
        regtrace = options.regtrace.split(",").collect();
    }

    let cctrace = options.cctrace;
    let solve = options.solve;
    let verbose = options.verbose;
    let mut cpu = Cpu::new(memsize,memtrace,regtrace,cctrace,solve,verbose);
    cpu.init();

    println!("{:#?}",cpu);
}