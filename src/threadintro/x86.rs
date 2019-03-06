use std::collections::HashMap;
use rand::prelude::*;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{BufReader, BufRead};

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
#[derive(Clone)]
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
    labels:HashMap<String,i32>,
    vars:HashMap<String,i32>,
    memory:HashMap<i32,String>,
    pmemory:HashMap<i32,String>,
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
            self.memory.entry(i).or_insert(0.to_string());
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
            if !self.pmemory.contains_key(&i) && self.memory.contains_key(&i) && self.memory[&i] != "0" {
                println!(" m[{}] {}",i,self.memory[&i]);
            }
        }
    }

    // INFORMING ABOUT THE HARDWARE

    fn get_regnum(&self, name:String) -> u8{
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
        return "".to_string();
    }

    fn get_regnums(&self) ->Vec<u8>{
        return self.regnums.clone();
    }

    fn get_condlist(&self) -> Vec<u8> {
        return self.condlist.clone();
    }

    fn get_reg(& self,reg:u8) ->i32{
        assert!(self.regnums.contains(&reg));
        return self.registers[&reg];
    }

    fn get_cond(& self,cond:u8) ->bool{
        assert!(self.condlist.contains(&cond));
        return self.conditions[&cond];
    }

    fn get_pc(& self) -> i32{
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

    fn halt(&mut self) ->i32 {
        return -1;
    }

    fn iyield(&mut self) ->i32 {
        return -2;
    }

    fn nop(&mut self) -> i32 {
        return 0;
    }

    fn rdump(&mut self) {
        print!("REGISTERS::");
        print!("ax: {} ",self.registers[&self.REG_AX]);
        print!("bx: {} ",self.registers[&self.REG_BX]);
        print!("cx: {} ",self.registers[&self.REG_CX]);
        print!("dx: {} ",self.registers[&self.REG_DX]);
    }

    fn mdump(&mut self,index:i32) {
        println!("m[{}] {}",index,self.memory[&index]);
    }

    fn move_i_to_r(&mut self,src:i32,dst:u8) -> i32 {
        self.registers.insert(dst,src);
        return 0;
    }

    //  memory: value, register, register

    fn move_i_to_m(&mut self,src:i32,value:i32,reg1:u8,reg2:u8) -> i32{
        let tmp = value + self.registers[&reg1] + self.registers[&reg2];
        self.memory.insert(tmp,src.to_string());
        return 0;
    }

    fn move_m_to_r(&mut self,value:i32,reg1:u8,reg2:u8,dst:u8) {
        let tmp = value + self.registers[&reg1] + self.registers[&reg2];
        self.registers.insert(dst,self.memory[&tmp].parse::<i32>().unwrap());
    }

    fn move_r_to_m(&mut self,src:u8,value:i32,reg1:u8,reg2:u8) -> i32{
        let tmp = value + self.registers[&reg1] + self.registers[&reg2];
        self.memory.insert(tmp,self.registers[&src].to_string());
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
        let old = self.memory[&tmp].parse::<i32>().unwrap();
        self.memory.insert(tmp,self.registers[&src].to_string());
        self.registers.insert(src,old);
        return 0;
    }

    fn fetchadd(&mut self,src:u8,value:i32,reg1:u8,reg2:u8) -> i32 {
        let tmp = value + self.registers[&reg1] + self.registers[&reg2];
        let old = self.memory[&tmp].parse::<i32>().unwrap();
        self.memory.insert(tmp,(self.memory[&tmp].parse::<i32>().unwrap()+self.registers[&src]).to_string());
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
        self.memory.insert(self.registers[&self.REG_SP],self.PC.to_string());
        self.PC = targ;
    }

    fn ret(&mut self) {
        self.PC = self.memory[&self.registers[&self.REG_SP]].parse::<i32>().unwrap();
        self.registers.insert(self.REG_SP,self.registers[&self.REG_SP] + 4);
    }

    //
    //  STACK and related
    //

    fn push_r(&mut self,reg:u8) -> i32{
        self.registers.insert(self.REG_SP,self.registers[&self.REG_SP] - 4);
        self.memory.insert(self.registers[&self.REG_SP],self.registers[&reg].to_string());
        return 0;
    }

    fn push_m(&mut self,value:i32,reg1:u8,reg2:u8) -> i32 {
        self.registers.insert(self.REG_SP,self.registers[&self.REG_SP] - 4);
        let tmp = value + self.registers[&reg1] + self.registers[&reg2];

        self.memory.insert(self.registers[&self.REG_SP],tmp.to_string());
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

    fn register_translate(&mut self,r:&String) ->u8{
        if self.regnames.contains_key(r) {
            return self.regnames[r];
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

    fn getarg(&mut self,arg:&String) -> (String,String){
        let mut tmp = arg.replace(",","");
        tmp = tmp.trim().to_string();
        if tmp.starts_with("$") {
            assert!(tmp.len() == 2,"correct form is $number (not {})",tmp);
            let value:Vec<&str> = tmp.split("$").collect();
            let nope = value[1].parse::<i32>();
            assert!(!nope.is_err());
            return (value[1].into(),"TYPE_IMMEDIATE".into());
        }else if tmp.starts_with("%"){
            let register:Vec<&str> = tmp.split('%').collect();
            return (self.register_translate(&register[1].to_string()).to_string(), "TYPE_REGISTER".to_string())
        }else if tmp.starts_with("("){
            let temp1:Vec<&str> = tmp.split("(").collect();
            let temp2:Vec<&str> = temp1[1].split(")").collect();
            let register:Vec<&str> = temp2[0].split("%").collect();

            return (format!("{},{},{}",0, self.register_translate(&register[1].to_string()), self.register_translate(&"zero".to_string())),"TYPE_MEMORY".to_string());

        }else if tmp.starts_with("."){
            let targ = tmp;
            return (targ,"TYPE_LABEL".to_string());
        }else if tmp.is_ascii() && tmp.parse::<i32>().is_err(){             // alpha digital   not very corrcet
            assert!(self.vars.contains_key(&tmp),"Variable {} is not declared" , tmp);
            let b = self.vars[&tmp];
            //return (format!("{},{},{}",&self.vars[&tmp],&self.register_translate(&"zero".to_string()),&self.register_translate(&"zero".to_string())),"TYPE_MEMORY".to_string());
            return (format!("{},{},{}",b, self.register_translate(&"zero".to_string()), self.register_translate(&"zero".to_string())),"TYPE_MEMORY".to_string());
        }else if !tmp.parse::<i32>().is_err() || tmp.starts_with("-") {
           let mut neg = 1;
           if tmp.starts_with("-") {
               tmp = tmp[1..].to_string();
               neg = -1;
           }
           let s:Vec<&str> = tmp.split("(").collect();
           if s.len() == 1 {
               let value = neg * tmp.parse::<i32>().unwrap();
               return (format!("{},{},{}",value, self.register_translate(&"zero".to_string()), self.register_translate(&"zero".to_string())),"TYPE_MEMORY".to_string());
           }else if s.len() == 2 {
               let value = neg * s[0].parse::<i32>().unwrap();
               let tt:Vec<&str> = s[1].split(")").collect();
               let t:Vec<&str> = tt[0].split(",").collect();
               if t.len() == 1 {
                   let register:Vec<&str> = t[0].split("&").collect();
                   return (
                       format!("{},{},{}" , value, self.register_translate(&register[1].to_string()), self.register_translate(&"zero".to_string())), "TYPE_MEMORY".to_string()
                       );
               }else if t.len() == 2 {
                   let register1:Vec<&str> = t[0].split("%").collect();
                   let register2:Vec<&str> = t[1].split("%").collect();
                   return (format!("{},{},{}",value,self.register_translate(&register1[1].to_string()), self.register_translate(&register2[1].to_string())),"TYPE_MEMORY".to_string());
               }
           }else {
               println!(" mov : bad argument [{}]",tmp);
               return ("".to_string(),"".to_string());
           }
        } 
        println!(" mov : bad argument [{}]",tmp);
        return ("".to_string(),"".to_string());
    }

    //
    // LOAD a program into memory
    // make it ready to execute
    //

    fn load(&mut self,infile:String,loadaddr:i32) {
        {   
            let mut pc = loadaddr;
            let path = Path::new(&infile);
            let fd = File::open(&path).expect("file not found");

            let bpc = loadaddr;
            let mut data = 100;

            let file = BufReader::new(&fd);
            for line in file.lines() {
                //println!("{}", line.unwrap());
                let mut cline:String = line.unwrap().trim().to_string();
                println!("pass 1 {}",cline);
                let ctmp:Vec<&str> = cline.split("#").collect();

                assert!(ctmp.len() == 1 || ctmp.len() ==2,"2");

                if ctmp.len() == 2 {
                    cline = ctmp[0].to_string();
                }
                println!("3");

                let tmp:Vec<&str> = cline.split(" ").collect();
                if tmp.len() == 0 {
                    continue;
                }
                println!("4");
                if tmp[0] == ".var" {
                    assert!(tmp.len() == 2);
                    assert!(!self.vars.contains_key(tmp[0]));
                    self.vars.insert(tmp[1].to_string(),data);
                    data += 4;
                    assert!(data<bpc,"Load address overrun by static data");
                    if self.verbose {
                        println!("ASSIGN VAR {} -->{} {}" ,  tmp[0], tmp[1], self.vars[tmp[1]]);
                    }
                }else if tmp[0].starts_with(".") {
                    assert!(tmp.len() == 1);
                    self.labels.insert(tmp[0].to_string(),pc);
                    if self.verbose {
                        println!("ASSIGN LABEL {} -->{}" ,  tmp[0],pc);
                    }
                }else {
                    pc+=1;
                }
            }
            if self.verbose{
                println!("");
            } 
            println!("5");
        }
        {
            println!("6");
            let mut pc = loadaddr;
            let path = Path::new(&infile);
            let fd = File::open(&path).expect("file not found");
            let file = BufReader::new(&fd);
            for line in file.lines() {
                //println!("{}", line.unwrap());
                let mut cline:String = line.unwrap().trim().to_string();
                println!("pass 2 {}",cline);
                let ctmp:Vec<&str> = cline.split("#").collect();
                assert!(ctmp.len() == 1 || ctmp.len() ==2);

                if ctmp.len() == 2 {
                    cline = ctmp[0].to_string();
                }

                let tmp:Vec<&str> = cline.splitn(2," ").collect();
                if tmp.len() == 0 {
                    continue;
                }


                if !cline.starts_with(".") {
                    let opcode = tmp[0];
                    self.pmemory.insert(pc,cline.clone());

                    // MAIN OPCODE LOOP

                    if opcode == "mov" {
                        let rtmp:Vec<&str> = tmp[1].splitn(2,",").collect();
                        assert!(tmp.len() == 2 && rtmp.len() == 2,"mov: needs two args, separated by commas [{}]",cline);
                        let arg1 = rtmp[0].trim();
                        let arg2 = rtmp[1].trim();
                        let (src,stype) = self.getarg(&arg1.to_string());
                        let (dst,dtype) = self.getarg(&arg2.to_string());
                        println!("MOV {}:{} , {}:{}",src,stype,dst,dtype);

                        if stype == "TYPE_MEMORY" || dtype == "TYPE_MEMORY" {
                            println!("bad mov: two memory arguments");
                            return;
                        }else if stype == "TYPE_IMMEDIATE" && dtype == "TYPE_IMMEDIATE" {
                            println!("bad mov: two immediate arguments");
                            return ;
                        }else if stype == "TYPE_IMMEDIATE" && dtype == "TYPE_REGISTER"{
                            self.memory.insert(pc,format!("self.move_i_to_r({}, {})" ,src, dst));
                        }else if stype == "TYPE_MEMORY" && dtype == "TYPE_REGISTER"{
                            
                        }else if stype == "TYPE_REGISTER" && dtype == "TYPE_MEMORY"{
                            
                        }else if stype == "TYPE_REGISTER" && dtype == "TYPE_REGISTER"{
                            
                        }else if stype == "TYPE_IMMEDIATE" && dtype == "TYPE_MEMORY"{
                            
                        }else {

                        }
                    }else if opcode == "pop" {

                    }else if opcode == "push" {
                        
                    }else if opcode == "call" {
                        
                    }else if opcode == "ret" {
                        
                    }else if opcode == "add" {
                        
                    }else if opcode == "sub" {
                        
                    }else if opcode == "fetchadd" {
                        
                    }else if opcode == "xchg" {
                        
                    }else if opcode == "test" {
                        
                    }else if opcode == "j" {
                        
                    }else if opcode == "jne" {
                        
                    }else if opcode == "je" {
                        
                    }else if opcode == "jlt" {
                        
                    }else if opcode == "jlte" {
                        
                    }else if opcode == "jgt" {
                        
                    }else if opcode == "jgte" {
                        
                    }else if opcode == "halt" {
                        
                    }else if opcode == "yield" {
                        
                    }else if opcode == "rdump" {
                        
                    }else if opcode == "mdump" {
                        
                    }else{

                    }

                    if self.verbose{
                        println!("pc:{} LOADING {} --> %{}",pc, self.pmemory[&pc], self.memory[&pc]);
                    } 

                    pc+=1;
                    
                }






            }
            if self.verbose{
                println!("");
            } 
        }
    }

    //  END: load

    fn print_headers(&mut self, procs:&mut Proclist,cctrace:bool) {
        if self.memtrace.len() > 0 {
            for m in &self.memtrace {
                let a = &m[0..1];
                if !a.to_string().parse::<i32>().is_err() {
                    print!("{}",m);
                }else {
                    assert!(self.vars.contains_key(m));
                    print! ("{}" , m);
                }
            }
        }

        if self.regtrace.len() > 0 {
            for r in self.regtrace.clone() {
                let rn:u8 = r.parse().unwrap();
                let s:String = self.get_regname(rn);
                print! ("{}" , s);
            }
            println!("");
        }

        if cctrace == true{
            print!(">= >  <= <  != =="); 
        }

        for i in 0..procs.getnum() {
            print! ("       Thread {}         " , i);
        }

    }

    fn print_trace(&mut self, newline:bool,cctrace:bool) {
        if self.memtrace.len() > 0 {
            for m in &self.memtrace {
                if self.compute {
                    let a = &m[0..1];
                    if !a.to_string().parse::<i32>().is_err() {
                        print!("{}",self.memory[&m.parse::<i32>().unwrap()]);
                    }else {
                        assert!(self.vars.contains_key(m));
                        print! ("{}" , self.memory[&self.vars[m]]);
                    }
                }else {
                    println!("?????");
                }
            }
            println!("");
        }

        if self.regtrace.len() > 0 {
            for r in self.regtrace.clone() {
                if self.compute{
                    println!("{}",self.registers[&r.parse::<u8>().unwrap()]);
                }else{
                    println!("?????");
                }
                println!("");
            }
            println!("");
        }

        if cctrace == true {
            for c in self.condlist.clone() {
                if self.compute {
                    if self.conditions[&c]{
                        print!("1");
                    }else{
                        print!("0");
                    }
                }else{
                    print!("?");
                }
            }
        }

        if (self.memtrace.len() > 0 || self.regtrace.len() > 0 || cctrace == true) && newline == true{
            println!("");
        }
            
 
        
    }

    fn setint(&mut self, intfreq:i32, intrand:bool) -> i32{
        if intrand == false {
            return intfreq;
        }
        let rand_x:f32 = rand::thread_rng().gen();
        return (rand_x * intfreq as f32)as i32 + 1;
    }

    fn run(&mut self ,mut procs:&mut Proclist,intfreq:i32,intrand:bool,cctrace:bool) ->i32 {
        let mut interrupt = self.setint(intfreq,intrand);
        let mut icount = 0;

        self.print_headers(procs,cctrace);
        self.print_trace(true,cctrace);

        loop {
            //  need thread ID of current process
            let tid = procs.getcurr().gettid() as i32;

            // FETCH

            let prevPC = self.PC;
            let instruction = &self.memory[&self.PC];
            self.PC += 1;

            // DECODE and EXECUTE
            // key: self.PC may be changed during eval; thus MUST be incremented BEFORE eval

            let rc = -1;
            //let rc = eval(instruction);   // ????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????

            //  tracing details: ALWAYS AFTER EXECUTION OF INSTRUCTION
            self.print_trace(false,cctrace);

            //  output: thread-proportional spacing followed by PC and instruction
            dospace(tid);
            println!("{}{}", prevPC,self.pmemory[&prevPC]);
            icount += 1;

            // halt instruction issueds

            if rc == -1 {
                procs.done();
                if procs.numdone() == procs.getnum() {
                    return icount;
                }
                procs.next();
                procs.restore();

                self.print_trace(false,cctrace);
                

                for i in 0..procs.getnum() {
                    print! ("----- Halt;Switch ----- ");
                }

                println!("");
            }

            interrupt -= 1;

            if interrupt == 0 || rc == -2 {
                interrupt = self.setint(intfreq, intrand);
                procs.save();
                procs.next();
                procs.restore();

                self.print_trace(false,cctrace);
                for i in 0..procs.getnum(){
                    print! ("------ Interrupt ------ ");
                }
                println! ("");
            }

            
        }
    }

}

struct Proclist{
    plist:Vec<Process>,
    curr:usize,
    active:usize,
}

impl Proclist {
    fn new() -> Proclist {
        Proclist {
            plist:Vec::new(),
            curr:0,
            active:0,
        }
    }

    fn done(&mut self) {
        self.plist[self.curr].setdone();
        self.active -= 1;
    }

    fn numdone(&mut self) -> usize {
        return self.plist.len() - self.active;
    }

    fn getnum(&mut self) -> usize {
        return self.plist.len();
    }

    fn add(&mut self,p:Process) {
        self.active += 1;
        self.plist.push(p);
    }

    fn getcurr(&self) -> Process{
        let p = self.plist[self.curr].clone();
        return p;
    }

    fn save(&mut self) {
        self.plist[self.curr].save();
    }

    fn restore(&mut self) {
        self.plist[self.curr].restore();
    }

    fn next(&mut self) {
        for i in self.curr+1..self.plist.len() {
            if self.plist[i].isdone() == false {
                self.curr = i;
                //return
            }
        }

        for i in 0..self.curr+1 {
            if self.plist[i].isdone() == false {
                self.curr = i;
                //return 
            }
        }
    }


}
#[derive(Clone)]
struct Process {
    cpu:Cpu,
    tid:usize,
    pc:i32,
    regs:HashMap<u8,i32>,
    cc:HashMap<u8,bool>,
    done:bool,
    stack:i32,
}

impl Process {
    fn new(cpu:Cpu,tid:usize,pc:i32,stackbottom:i32) -> Process {
        Process {
            cpu:cpu,
            tid:tid,
            pc:pc,
            regs:HashMap::new(),
            cc:HashMap::new(),
            done:false,
            stack:stackbottom,
        }
    }

    fn init(&mut self,reginit:String) {
        for r in self.cpu.get_regnums() {
            self.regs.entry(r).or_insert(0);
        }

        if reginit != "" {
            let regsinitlist:Vec<&str> = reginit.split(":").collect();
            for r in regsinitlist {
                let tmp:Vec<&str> = r.split("=").collect();
                assert!(tmp.len() == 2);
                self.regs.entry(self.cpu.get_regnum(tmp[0].to_string())).or_insert(tmp[1].parse().unwrap());
            }
        }

        for c in self.cpu.get_condlist() {
            self.cc.entry(c).or_insert(false);
        }

        self.regs.insert(self.cpu.get_regnum("sp".to_string()),self.stack);

    }

    fn gettid (&mut self) -> usize {
        return self.tid;
    }

    fn save(&mut self) {
        self.pc = self.cpu.get_pc();

        for c in self.cpu.get_condlist() {
            self.cc.insert(c,self.cpu.get_cond(c));
        }

        for r in self.cpu.get_regnums() {
            self.regs.insert(r,self.cpu.get_reg(r));
        }
    }

    fn restore(&mut self) {
        self.cpu.set_pc(self.pc);
        for c in self.cpu.get_condlist() {
            self.cpu.set_cond(c,self.cc[&c]);
        }

        for r in self.cpu.get_regnums() {
            self.cpu.set_reg(r,self.regs[&r]);
        }

    }

    fn setdone(&mut self) {
        self.done = true;
    }

    fn isdone(&mut self) -> bool{
        return self.done == true;
    }
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


    println! ("ARG seed {}",                options.seed);
    println! ("ARG numthreads {}",          options.numthreads);
    println! ("ARG program {}",             options.progfile);
    println! ("ARG interrupt frequency {}", options.intfreq);
    println! ("ARG interrupt randomness {}",options.intrand);
    println! ("ARG argv {}",                options.argv);
    println! ("ARG load address {}",        options.loadaddr);
    println! ("ARG memsize {}",             options.memsize);
    println! ("ARG memtrace {}",            options.memtrace);
    println! ("ARG regtrace {}",            options.regtrace);
    println! ("ARG cctrace {}",             options.cctrace);
    println! ("ARG printstats {}",          options.printstats);
    println! ("ARG verbose {}",             options.verbose);
    println! ("");

    println!("{:#?}",options);

    let seed = options.seed;
    let numthreads = options.numthreads;
    let intfreq = options.intfreq;
    let intrand = options.intrand;
    let progfile = options.progfile;
    let argv:Vec<&str> = options.argv.split(",").collect();

    let loadaddr = options.loadaddr;
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
    let printstats = options.printstats;
    let verbose = options.verbose;


    let mut cpu = Cpu::new(memsize,memtrace,regtrace,cctrace,solve,verbose);
    cpu.init();

    cpu.load(progfile,loadaddr);
    //println!("{:#?}",cpu);

    // process list

    // let mut procs = Proclist::new();

    // let mut pid = 0;

    // let mut stack = memsize * 1000;

    // for t in 0..numthreads {
    //     let mut arg;
    //     if argv.len() > 1 {
    //         arg = argv[pid];
    //     }else {
    //         arg = argv[0];
    //     }
    //     let mut p =  Process::new(cpu.clone(), pid, loadaddr, stack);
    //     p.init(arg.to_string());
    //     procs.add(p);
    //     stack -= 1000;
    //     pid+=1;
    // }

    // procs.restore();

    // time
    // let t1;
    // let ic;
    // let t2;

    // if printstats {
    //     println!("");
    //     println!("STATS:: Instructions    {}", ic);
    //     println!("STATS:: Emulation Rate  {} kinst/sec", ic/(t2-t1)/1000);
    // }
}