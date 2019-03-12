// not done

use std::collections::HashMap;
use rand::prelude::*;
enum Process_Switch_Behavior {
    SWITCH_ON_IO,
    SWITCH_ON_END,
}
enum IO_Done_Behavior {
    IO_RUN_LATER,
    IO_RUN_IMMEDIATE,
}
enum Process_States {
    STATE_RUNNING,
    STATE_READY,
    STATE_DONE,
    STATE_WAIT,
}
enum Process_Do {
    DO_COMPUTE,
    DO_IO,
}

const PROC_CODE:&str = "code_";
const PROC_PC:&str = "pc_";
const PROC_ID:&str = "pid_";
const PROC_STATE:&str = "proc_state_";

struct Scheduler {
    seed:i32,
    process_info:HashMap<usize,HashMap<String,Vec<String>>>,
    //process_switch_behavior:Process_Switch_Behavior,
    process_switch_behavior:String,
    io_done_behavior:String,
    io_length:i32,
    curr_proc:i32,
    io_finish_times:HashMap<usize,Vec<i32>>,
}

impl Scheduler {
    // fn new (process_switch_behavior:Process_Switch_Behavior,io_done_behavior:IO_Done_Behavior,io_length:u32) -> Scheduler {
    fn new (process_switch_behavior:String,io_done_behavior:String,io_length:i32) -> Scheduler {
        Scheduler {
            seed:0,
            process_info : HashMap::new(),
            process_switch_behavior : String::from(""),
            io_done_behavior : String::from(""),
            io_length : io_length,
            curr_proc:0,
            io_finish_times:HashMap::new(),
        }
    }

    fn new_process(&mut self) -> usize{
        let proc_id = self.process_info.len();
        self.process_info.entry(proc_id).or_insert(HashMap::new());
        self.process_info.get_mut(&proc_id).unwrap().entry(PROC_PC.to_string()).or_insert(vec!["0".to_string()]);
        self.process_info.get_mut(&proc_id).unwrap().entry(PROC_ID.to_string()).or_insert(vec![proc_id.to_string()]);
        self.process_info.get_mut(&proc_id).unwrap().entry(PROC_CODE.to_string());
        self.process_info.get_mut(&proc_id).unwrap().entry(PROC_STATE.to_string()).or_insert(vec!["STATE_READY".to_string()]);
        proc_id
    }

    fn load(&mut self,program_description:&str) {
        let proc_id = self.new_process();
        let tmp:Vec<&str> = program_description.split(':').collect();
        if tmp.len() != 2 {
            println!("Bad description (%s): Must be number <x:y>");
            println!("where X is the number of instructions");
            println!("and Y is the percent change that an instruction is CPU not IO");
            return
        }

        let num_instructions :i32 = tmp[0].parse().unwrap();
        let chance_cpu:f32 = tmp[1].parse::<f32>().unwrap() / 100.0;
        let seed_u8 = self.seed as u8;
        let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

         let mut rng = SmallRng::from_seed(seed);
        for i in 0..num_instructions {
            let rand_x :f32 = rng.gen();
            if rand_x < chance_cpu {
                self.process_info.get_mut(&proc_id).unwrap().get_mut(&(PROC_CODE.to_string())).unwrap().push("DO_COMPUTE".to_string());
            }else
            {
                self.process_info.get_mut(&proc_id).unwrap().get_mut(&(PROC_CODE.to_string())).unwrap().push("DO_IO".to_string());
            }
        }
    }

    fn move_to_ready(&mut self,expected:&str,mut pid:i32) {
        if pid == -1 {
            pid = self.curr_proc;
        }
        assert_eq!(self.process_info[&(pid as usize)][PROC_STATE][0] , expected.to_string());
        self.process_info.get_mut(&(pid as usize)).unwrap().get_mut(&(PROC_STATE.to_string())).unwrap().push("STATE_READY".to_string());
    }

    fn move_to_wait(&mut self,expected:&str,) {
        assert!(self.process_info[&(self.curr_proc as usize)][PROC_STATE][0] == expected);
        self.process_info.get_mut(&(self.curr_proc as usize)).unwrap().get_mut(&(PROC_STATE.to_string())).unwrap().push("STATE_WAIT".to_string());
    }

        fn move_to_running(&mut self,expected:&str,) {
        assert!(self.process_info[&(self.curr_proc as usize)][PROC_STATE][0] == expected);
        self.process_info.get_mut(&(self.curr_proc as usize)).unwrap().get_mut(&(PROC_STATE.to_string())).unwrap().push("STATE_RUNNING".to_string());
    }

        fn move_to_done(&mut self,expected:&str,) {
        assert!(self.process_info[&(self.curr_proc as usize)][PROC_STATE][0] == expected);
        self.process_info.get_mut(&(self.curr_proc as usize)).unwrap().get_mut(&(PROC_STATE.to_string())).unwrap().push("STATE_DONE".to_string());
    }

    fn next_proc(&mut self,pid:i32) {
        if pid != -1 {
            self.curr_proc = pid;
            self.move_to_running("STATE_READY");
            return 
        }

        for pid in self.curr_proc+1..self.process_info.len() as i32 {
            if self.process_info[&(pid as usize)][PROC_STATE][0] == "STATE_READY" {
                self.curr_proc = pid;
                self.move_to_running("STATE_READY");
            }
            return ;
        }

        for pid in 0..self.curr_proc+1 {
            if self.process_info[&(pid as usize)][PROC_STATE][0] == "STATE_READY" {
                self.curr_proc = pid;
                self.move_to_running("STATE_READY");
            }
            return ;
        }
    }

    fn get_num_processes(&self) -> usize{
        return self.process_info.len();
    }

    fn get_num_instructions(&self,pid:i32) -> usize{
        return self.process_info[&(pid as usize)][PROC_CODE].len();
    }

    fn get_instructions(&self,pid:i32,index:usize) -> String{
        return self.process_info[&(pid as usize)][PROC_CODE][index].clone();
    }

    fn get_num_active(&self) -> i32{
        let mut num_active = 0;
        for pid in 0..self.process_info.len() {
            if self.process_info[&(pid)][PROC_STATE][0] != "STATE_DONE" {
                num_active +=1;
            }
        }

        return num_active;
    }

    fn get_num_runnable(&self)->i32 {
        let mut num_active = 0;
        for pid in 0..self.process_info.len() {
            if self.process_info[&(pid)][PROC_STATE][0] == "STATE_READY"  ||  self.process_info[&(pid)][PROC_STATE][0] == "STATE_RUNNING" {
                num_active +=1;
            }
        }
        return num_active;
    }

    fn get_ios_in_flight(&self,current_time:i32) -> i32 {
        let mut num_in_flight = 0;
        for pid in 0..self.process_info.len() {
            for t in self.io_finish_times[&pid].clone() {
                if t > current_time {
                    num_in_flight += 1;
                }
            }
        }
        return num_in_flight;
    }

    fn check_if_done(&mut self) {
        if self.process_info[&(self.curr_proc as usize)][PROC_CODE].len() == 0 {
            if self.process_info[&(self.curr_proc as usize)][PROC_STATE][0] =="STATE_RUNNING" {
                self.move_to_done("STATE_RUNNING");
                self.next_proc(-1);
            }
        }
    }




    fn run(&mut self) -> (i32,i32,i32){
        let mut clock_tick = 0;

        if self.process_info.len() == 0 {
            return (0,0,0);
        }

        self.curr_proc = 0;
        self.move_to_running("STATE_READY");

        print!("Time");
        for pid in 0..self.process_info.len() {
            print!("   PID:{}   " ,pid);
        }
        print! ("       CPU        ");
        print! ("       IOs     ");
        println! ("");

        let mut io_busy = 0;
        let mut cpu_busy = 0;

        while self.get_num_active() > 0 {
            clock_tick += 1;

            let mut io_done = false;

            for pid in 0..self.process_info.len() {
                if self.io_finish_times[&pid].contains(&clock_tick) {
                    io_done = true;
                    self.move_to_ready("STATE_WAIT", pid as i32);
                    if self.io_done_behavior == "IO_RUN_IMMEDIATE".to_string(){
                        if self.curr_proc != pid as i32 {
                            if self.process_info[&(self.curr_proc as usize)][PROC_STATE][0] == "STATE_RUNNING" {
                                self.move_to_ready("STATE_RUNNING",-1);
                            }
                        }
                        self.next_proc(pid as i32);
                    }else {
                        if self.process_switch_behavior == "SWITCH_ON_END".to_string() {
                            self.next_proc(pid as i32);
                        }

                        if self.get_num_runnable() == 1 {
                            self.next_proc(pid as i32);
                        }
                    }
                    self.check_if_done();
                }
            }

            let mut instruction_to_execute = "".to_string();

            if self.process_info[&(self.curr_proc as  usize)][PROC_STATE][0] == "STATE_RUNNING" &&  self.process_info[&(self.curr_proc as  usize)][PROC_CODE].len() > 0 {
                instruction_to_execute = self.process_info.get_mut(&(self.curr_proc as usize)).unwrap().get_mut(PROC_CODE).unwrap().remove(0);
                cpu_busy += 1;
            }

            if io_done {
                print! (" {} *" ,clock_tick);
            }else {
                print! (" {} " ,clock_tick);
            }

            for pid in 0..self.process_info.len() {
                if pid == self.curr_proc as usize && instruction_to_execute != "" {
                    print! ("RUN: {} " ,instruction_to_execute);
                }else {
                    print! ("RUN: {} " ,self.process_info[&pid][PROC_STATE][0]);
                }
            }

            if  instruction_to_execute == "" {
                print! ("               ");
            }else {
                print! ("               1");
            }

            let num_outstanding = self.get_ios_in_flight(clock_tick);

            if num_outstanding > 0 {
                print! ("   {}  " ,num_outstanding);
                io_busy += 1;
            }else {
                print! ("      " );
            }

            println!("");
 

            if instruction_to_execute == "DO_IO" {
                self.move_to_wait("STATE_RUNNING");
                self.io_finish_times.get_mut(&(self.curr_proc as usize)).unwrap().push(clock_tick + self.io_length);
                if self.process_switch_behavior ==  "SWITCH_ON_IO" {
                     self.next_proc(-1);
                }
            }



        self.check_if_done();

            
        }
        return (cpu_busy,io_busy,clock_tick);
    }

}

struct process_option {
    seed:i32,
    process_list:String,
    io_length:i32,
    // process_switch_behavior:Process_Switch_Behavior,
    process_switch_behavior:String,
    // io_done_behavior:IO_Done_Behavior,
    io_done_behavior:String,
    solve:bool,
    print_stats:bool,
}

impl process_option {
    fn new () -> process_option{
        process_option {
            seed:0,
            process_list:String::from(""),
            io_length:5,
            // process_switch_behavior:Process_Switch_Behavior::SWITCH_ON_IO,
            // io_done_behavior:IO_Done_Behavior::IO_RUN_LATER,
            process_switch_behavior:String::from("SWITCH_ON_IO"),
            io_done_behavior:String::from("IO_RUN_LATER"),
            solve:false,
             print_stats:false,
        }
    }
}

pub fn process_run_op_parse (op_vec:Vec<&str>) {
    let mut proc_op = process_option::new();
    let mut i =1;
    while i<op_vec.len() {
        match op_vec[i] {
            "-s" =>{proc_op.seed = op_vec[i+1].parse().unwrap();i = i+2;},
            "-l" =>{proc_op.process_list = op_vec[i+1].to_string();i = i+2;},
            "-L" =>{proc_op.io_length = op_vec[i+1].parse().unwrap();i=i+2;},
            "-S" =>{
                match op_vec[i+1] {
                        "SWITCH_ON_IO" => proc_op.process_switch_behavior = "SWITCH_ON_IO".to_string(),
                        "SWITCH_ON_END" => proc_op.process_switch_behavior = "SWITCH_ON_END".to_string(),
                        _ => println!("Wrong Input!"),
                    }
                    i=i+2;
                },
            "-I" =>{
                 match op_vec[i+1] {
                        "IO_RUN_LATER" => proc_op.io_done_behavior = "IO_RUN_LATER".to_string(),
                        "IO_RUN_IMMEDIATE" => proc_op.io_done_behavior = "IO_RUN_IMMEDIATE".to_string(),
                        _ => println!("Wrong Input!"),
                    }
                    i=i+2;
                },
            "-c" =>{proc_op.solve = true;i=i+1;},
            "-p" =>{proc_op.print_stats = true;i=i+1;},
            _ => println!("process_run_op_parse match wrong!!"),
        }
    }
    //println!("{},{},{:#?},{:#?},{},{},{}",proc_op.seed,proc_op.process_list,proc_op.process_switch_behavior,proc_op.io_done_behavior,proc_op.io_length,proc_op.solve,proc_op.print_stats);
    println!("{},{},{},{},{}",proc_op.seed,proc_op.process_list,proc_op.io_length,proc_op.solve,proc_op.print_stats);
    //let mut s = Scheduler::new(proc_op.process_switch_behavior,proc_op.io_done_behavior,proc_op.io_length);
    execute_process_op(proc_op);
    //s.test();
}

fn execute_process_op(options:process_option) {
    let mut s = Scheduler::new(options.process_switch_behavior,options.io_done_behavior,options.io_length);
    for p in options.process_list.split(",") {
        s.load(p);
    }

    if options.solve == false {
        
    }

    let (cpu_busy, io_busy, clock_tick) = s.run();

    if options.print_stats{
        println! ("");
        println! ("Stats: Total Time {}" , clock_tick);
        println! ("Stats: CPU Busy {} ({})",  cpu_busy, 100.0 * (cpu_busy/clock_tick) as f64);
        println! ("Stats: IO Busy  {} ({})" , io_busy, 100.0 * (io_busy/clock_tick) as f64);
        println! ("");
    }
    
}