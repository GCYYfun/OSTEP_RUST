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
    process_switch_behavior:Process_Switch_Behavior,
    io_done_behavior:IO_Done_Behavior,
    io_length:u32,
    curr_proc:i32,
}

impl Scheduler {
    fn new (process_switch_behavior:Process_Switch_Behavior,io_done_behavior:IO_Done_Behavior,io_length:u32) -> Scheduler {
        Scheduler {
            seed:0,
            process_info : HashMap::new(),
            process_switch_behavior : process_switch_behavior,
            io_done_behavior : io_done_behavior,
            io_length : io_length,
            curr_proc:0,
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




    fn run(&mut self) {

    }

}

struct process_option {
    seed:i32,
    process_list:String,
    io_length:u32,
    process_switch_behavior:Process_Switch_Behavior,
    io_done_behavior:IO_Done_Behavior,
    solve:bool,
    print_stats:bool,
}

impl process_option {
    fn new () -> process_option{
        process_option {
            seed:0,
            process_list:String::from(""),
            io_length:5,
            process_switch_behavior:Process_Switch_Behavior::SWITCH_ON_IO,
            io_done_behavior:IO_Done_Behavior::IO_RUN_LATER,
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
                        "SWITCH_ON_IO" => proc_op.process_switch_behavior = Process_Switch_Behavior::SWITCH_ON_IO,
                        "SWITCH_ON_END" => proc_op.process_switch_behavior = Process_Switch_Behavior::SWITCH_ON_END,
                        _ => println!("Wrong Input!"),
                    }
                    i=i+2;
                },
            "-I" =>{
                 match op_vec[i+1] {
                        "IO_RUN_LATER" => proc_op.io_done_behavior = IO_Done_Behavior::IO_RUN_LATER,
                        "IO_RUN_IMMEDIATE" => proc_op.io_done_behavior = IO_Done_Behavior::IO_RUN_IMMEDIATE,
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
}