// not done

use rand::{Rng, SeedableRng};
use std::collections::HashMap;

const HELP: &str = include_str!("help.txt");

const SCHED_SWITCH_ON_IO: &str = "SWITCH_ON_IO";
const SCHED_SWITCH_ON_END: &str = "SWITCH_ON_END";

const IO_RUN_LATER: &str = "IO_RUN_LATER";
const IO_RUN_IMMEDIATE: &str = "IO_RUN_IMMEDIATE";

const STATE_RUNNING: &str = "RUNNING";
const STATE_READY: &str = "READY";
const STATE_DONE: &str = "DONE";
const STATE_WAIT: &str = "WAITING";

const DO_COMPUTE: &str = "cpu";
const DO_IO: &str = "io";
const DO_IO_DONE: &str = "io_done";

const PROC_CODE: &str = "code_";
const PROC_PC: &str = "pc_";
const PROC_ID: &str = "pid_";
const PROC_STATE: &str = "proc_state_";

struct Scheduler {
    seed: u64,
    process_info: HashMap<usize, HashMap<String, Vec<String>>>,
    process_switch_behavior: String,
    io_done_behavior: String,
    io_length: i32,
    curr_proc: i32,
    io_finish_times: HashMap<usize, Vec<i32>>,
}

impl Scheduler {
    fn new(process_switch_behavior: String, io_done_behavior: String, io_length: i32) -> Scheduler {
        Scheduler {
            seed: 0,
            process_info: HashMap::new(),
            process_switch_behavior: process_switch_behavior,
            io_done_behavior: io_done_behavior,
            io_length: io_length,
            curr_proc: 0,
            io_finish_times: HashMap::new(),
        }
    }

    fn new_process(&mut self) -> usize {
        let proc_id = self.process_info.len();
        self.process_info.entry(proc_id).or_insert(HashMap::new());
        self.process_info
            .get_mut(&proc_id)
            .unwrap()
            .entry(PROC_PC.to_string())
            .or_insert(vec!["0".to_string()]);
        self.process_info
            .get_mut(&proc_id)
            .unwrap()
            .entry(PROC_ID.to_string())
            .or_insert(vec![proc_id.to_string()]);
        self.process_info
            .get_mut(&proc_id)
            .unwrap()
            .entry(PROC_CODE.to_string())
            .or_insert(vec![]);
        self.process_info
            .get_mut(&proc_id)
            .unwrap()
            .entry(PROC_STATE.to_string())
            .or_insert(vec![STATE_READY.to_string()]);
        proc_id
    }

    fn load_program(&mut self, program: &str) {
        let proc_id = self.new_process();
        let tmp: Vec<&str> = program.split(",").collect();
        for line in tmp {
            if line.starts_with("c") {
                let num = line.get(1..).unwrap().len();
                for _i in 0..num {
                    self.process_info
                        .get_mut(&proc_id)
                        .unwrap()
                        .get_mut(&(PROC_CODE.to_string()))
                        .unwrap()
                        .push(DO_COMPUTE.to_string());
                }
            } else if line.starts_with("i") {
                self.process_info
                    .get_mut(&proc_id)
                    .unwrap()
                    .get_mut(&(PROC_CODE.to_string()))
                    .unwrap()
                    .push(DO_IO.to_string());
                self.process_info
                    .get_mut(&proc_id)
                    .unwrap()
                    .get_mut(&(PROC_CODE.to_string()))
                    .unwrap()
                    .push(DO_IO_DONE.to_string());
            } else {
                println!("bad opcode {} (should be c or i)", line);
                return;
            }
        }
    }

    fn load(&mut self, program_description: &str) {
        let proc_id = self.new_process();
        let tmp: Vec<&str> = program_description.split(":").collect();
        if tmp.len() != 2 {
            println!("Bad description (%s): Must be number <x:y>");
            println!("where X is the number of instructions");
            println!("and Y is the percent change that an instruction is CPU not IO");
            return;
        }

        let num_instructions: i32 = tmp[0].parse().unwrap();
        let chance_cpu: f32 = tmp[1].parse::<f32>().unwrap() / 100.0;

        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(self.seed);
        for _i in 0..num_instructions {
            let rand_x: f32 = rng.gen::<f32>();
            if rand_x < chance_cpu {
                self.process_info
                    .get_mut(&proc_id)
                    .unwrap()
                    .get_mut(&(PROC_CODE.to_string()))
                    .unwrap()
                    .push(DO_COMPUTE.to_string());
            } else {
                self.process_info
                    .get_mut(&proc_id)
                    .unwrap()
                    .get_mut(&(PROC_CODE.to_string()))
                    .unwrap()
                    .push(DO_IO.to_string());
                self.process_info
                    .get_mut(&proc_id)
                    .unwrap()
                    .get_mut(&(PROC_CODE.to_string()))
                    .unwrap()
                    .push(DO_IO_DONE.to_string());
            }
        }
    }

    fn move_to_ready(&mut self, expected: &str, mut pid: i32) {
        if pid == -1 {
            pid = self.curr_proc;
        }
        assert_eq!(
            self.process_info[&(pid as usize)][PROC_STATE][0],
            expected.to_string()
        );
        self.process_info
            .get_mut(&(pid as usize))
            .unwrap()
            .get_mut(&(PROC_STATE.to_string()))
            .unwrap()[0] = STATE_READY.to_string();
    }

    fn move_to_wait(&mut self, expected: &str) {
        assert!(self.process_info[&(self.curr_proc as usize)][PROC_STATE][0] == expected);
        self.process_info
            .get_mut(&(self.curr_proc as usize))
            .unwrap()
            .get_mut(&(PROC_STATE.to_string()))
            .unwrap()[0] = STATE_WAIT.to_string();
    }

    fn move_to_running(&mut self, expected: &str) {
        assert!(self.process_info[&(self.curr_proc as usize)][PROC_STATE][0] == expected);
        self.process_info
            .get_mut(&(self.curr_proc as usize))
            .unwrap()
            .get_mut(&(PROC_STATE.to_string()))
            .unwrap()[0] = STATE_RUNNING.to_string();
    }

    fn move_to_done(&mut self, expected: &str) {
        assert!(self.process_info[&(self.curr_proc as usize)][PROC_STATE][0] == expected);
        self.process_info
            .get_mut(&(self.curr_proc as usize))
            .unwrap()
            .get_mut(&(PROC_STATE.to_string()))
            .unwrap()[0] = STATE_DONE.to_string();
    }

    fn next_proc(&mut self, pid: i32) {
        if pid != -1 {
            self.curr_proc = pid;
            self.move_to_running(STATE_READY);
            return;
        }

        for pid in self.curr_proc + 1..self.process_info.len() as i32 {
            if self.process_info[&(pid as usize)][PROC_STATE][0] == STATE_READY {
                self.curr_proc = pid;
                self.move_to_running(STATE_READY);
                return;
            }
        }

        for pid in 0..self.curr_proc + 1 {
            if self.process_info[&(pid as usize)][PROC_STATE][0] == STATE_READY {
                self.curr_proc = pid;
                self.move_to_running(STATE_READY);
                return;
            }
        }
        return;
    }

    fn get_num_processes(&self) -> usize {
        return self.process_info.len();
    }

    fn get_num_instructions(&self, pid: i32) -> usize {
        return self.process_info[&(pid as usize)][PROC_CODE].len();
    }

    fn get_instructions(&self, pid: i32, index: usize) -> String {
        return self.process_info[&(pid as usize)][PROC_CODE][index].clone();
    }

    fn get_num_active(&self) -> i32 {
        let mut num_active = 0;
        for pid in 0..self.process_info.len() {
            if self.process_info[&pid][PROC_STATE][0] != STATE_DONE {
                num_active += 1;
            }
        }

        return num_active;
    }

    fn get_num_runnable(&self) -> i32 {
        let mut num_active = 0;
        for pid in 0..self.process_info.len() {
            if self.process_info[&(pid)][PROC_STATE][0] == STATE_READY
                || self.process_info[&(pid)][PROC_STATE][0] == STATE_RUNNING
            {
                num_active += 1;
            }
        }
        return num_active;
    }

    fn get_ios_in_flight(&self, current_time: i32) -> i32 {
        let mut num_in_flight = 0;
        for pid in 0..self.process_info.len() {
            if self.io_finish_times.contains_key(&pid) {
                for t in self.io_finish_times[&pid].clone() {
                    if t > current_time {
                        num_in_flight += 1;
                    }
                }
            }
        }
        return num_in_flight;
    }

    fn check_if_done(&mut self) {
        if self.process_info[&(self.curr_proc as usize)][PROC_CODE].len() == 0 {
            if self.process_info[&(self.curr_proc as usize)][PROC_STATE][0] == STATE_RUNNING {
                self.move_to_done(STATE_RUNNING);
                self.next_proc(-1);
            }
        }
    }

    fn run(&mut self) -> (i32, i32, i32) {
        let mut clock_tick = 0i32;

        if self.process_info.len() == 0 {
            return (0, 0, 0);
        }

        for pid in 0..self.process_info.len() {
            self.io_finish_times.entry(pid).or_insert(vec![]);
        }

        self.curr_proc = 0;
        self.move_to_running(STATE_READY);

        print!("{:<12}", "Time");
        for pid in 0..self.process_info.len() {
            print!("{:>12}:{:<12}", "PID", pid);
        }
        print!("{:^24}", "CPU");
        print!("{:^24}", "IOs");
        println!("");

        let mut io_busy = 0;
        let mut cpu_busy = 0;

        while self.get_num_active() > 0 {
            clock_tick += 1;

            let mut io_done = false;
            for pid in 0..self.process_info.len() {
                if self.io_finish_times.contains_key(&pid)
                    && self
                        .io_finish_times
                        .get(&pid)
                        .unwrap()
                        .contains(&clock_tick)
                {
                    io_done = true;
                    self.move_to_ready(STATE_WAIT, pid as i32);
                    if self.io_done_behavior == IO_RUN_IMMEDIATE.to_string() {
                        if self.curr_proc != pid as i32 {
                            if self.process_info[&(self.curr_proc as usize)][PROC_STATE][0]
                                == STATE_RUNNING
                            {
                                self.move_to_ready(STATE_RUNNING, -1);
                            }
                        }
                        self.next_proc(pid as i32);
                    } else {
                        if self.process_switch_behavior == SCHED_SWITCH_ON_END.to_string() {
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

            if self.process_info[&(self.curr_proc as usize)][PROC_STATE][0] == STATE_RUNNING
                && self.process_info[&(self.curr_proc as usize)][PROC_CODE].len() > 0
            {
                instruction_to_execute = self
                    .process_info
                    .get_mut(&(self.curr_proc as usize))
                    .unwrap()
                    .get_mut(PROC_CODE)
                    .unwrap()
                    .remove(0);
                cpu_busy += 1;
            }
            if io_done {
                print!("{}{:<12}", clock_tick, "*");
            } else {
                print!("{:<12}", clock_tick);
            }

            for pid in 0..self.process_info.len() {
                if pid == self.curr_proc as usize && instruction_to_execute != "" {
                    print!("{:>12}: {:<12}", "RUN", instruction_to_execute);
                } else {
                    print!(
                        "{:>12}: {:<12}",
                        "RUN", self.process_info[&pid][PROC_STATE][0]
                    );
                }
            }

            if instruction_to_execute == "" {
                print!("{:^24}", "_");
            } else {
                print!("{:^24}", "1");
            }

            let num_outstanding = self.get_ios_in_flight(clock_tick);

            if num_outstanding > 0 {
                print!("{:^24}", num_outstanding);
                io_busy += 1;
            } else {
                print!("{:^24}", "_");
            }

            println!("");

            if instruction_to_execute == DO_IO {
                self.move_to_wait(STATE_RUNNING);
                self.io_finish_times
                    .get_mut(&(self.curr_proc as usize))
                    .unwrap()
                    .push(clock_tick + self.io_length + 1);
                if self.process_switch_behavior == SCHED_SWITCH_ON_IO {
                    self.next_proc(-1);
                }
            }

            self.check_if_done();
        }
        return (cpu_busy, io_busy, clock_tick);
    }
}

struct ProcessOption {
    seed: u64,
    program: String,
    process_list: String,
    io_length: i32,
    process_switch_behavior: String,
    io_done_behavior: String,
    solve: bool,
    print_stats: bool,
}

impl ProcessOption {
    fn new() -> ProcessOption {
        ProcessOption {
            seed: 0,
            program: String::from(""),
            process_list: String::from(""),
            io_length: 5,
            process_switch_behavior: String::from(SCHED_SWITCH_ON_IO),
            io_done_behavior: String::from(IO_RUN_LATER),
            solve: false,
            print_stats: false,
        }
    }
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut proc_op = ProcessOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" | "--seed" => {
                proc_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-P" | "--program" => {
                proc_op.program = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-l" | "--processlist" => {
                proc_op.process_list = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-L" | "--iolength" => {
                proc_op.io_length = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-S" | "--switch" => {
                match op_vec[i + 1] {
                    SCHED_SWITCH_ON_IO => {
                        proc_op.process_switch_behavior = SCHED_SWITCH_ON_IO.to_string()
                    }
                    SCHED_SWITCH_ON_END => {
                        proc_op.process_switch_behavior = SCHED_SWITCH_ON_END.to_string()
                    }
                    _ => println!("Wrong Input!"),
                }
                i = i + 2;
            }
            "-I" | "--iodone" => {
                match op_vec[i + 1] {
                    IO_RUN_LATER => proc_op.io_done_behavior = IO_RUN_LATER.to_string(),
                    IO_RUN_IMMEDIATE => proc_op.io_done_behavior = IO_RUN_IMMEDIATE.to_string(),
                    _ => println!("Wrong Input!"),
                }
                i = i + 2;
            }
            "-c" => {
                proc_op.solve = true;
                i = i + 1;
            }
            "-p" | "--printstats" => {
                proc_op.print_stats = true;
                i = i + 1;
            }
            _ => {
                println!("process_run_op_parse match wrong!!");
                i = i + 1;
            }
        }
    }

    // println!(
    //     "ARGS:↓\n\nseed:{},\nprocess_list:{},\nio_length:{},\nsolve:{},\nprint_stats:{}\n",
    //     proc_op.seed, proc_op.process_list, proc_op.io_length, proc_op.solve, proc_op.print_stats
    // );

    execute_process_op(proc_op);
}

fn execute_process_op(options: ProcessOption) {
    let mut s = Scheduler::new(
        options.process_switch_behavior.clone(),
        options.io_done_behavior.clone(),
        options.io_length,
    );

    if options.program != "".to_string() {
        for p in options.program.split(":") {
            s.load_program(p);
        }
    } else {
        for p in options.process_list.split(",") {
            s.load(p);
        }
    }

    if options.solve == false {
        println!("Produce a trace of what would happen when you run these processes:");
        for pid in 0..s.get_num_processes() {
            println!("Process {} ", pid);
            for inst in 0..s.get_num_instructions(pid as i32) {
                println!(" {} ", s.get_instructions(pid as i32, inst));
            }
            println!();
        }
        println!("Important behaviors:");
        print!("  System will switch when ");
        if SCHED_SWITCH_ON_IO == options.process_switch_behavior {
            println!("the current process is FINISHED or ISSUES AN IO");
        } else {
            println!("the current process is FINISHED");
        }

        print!("  After IOs, the process issuing the IO will ");
        if options.io_done_behavior == IO_RUN_IMMEDIATE {
            println!("run IMMEDIATELY");
        } else {
            println!("run LATER (when it is its turn)")
        }
        println!();
        return;
    }

    let (cpu_busy, io_busy, clock_tick) = s.run();

    if options.print_stats {
        println!("");
        println!("Stats: Total Time {}", clock_tick);
        println!(
            "Stats: CPU Busy {} ({:.2}%)",
            cpu_busy,
            100.0 * (cpu_busy as f64 / clock_tick as f64)
        );
        println!(
            "Stats: IO Busy  {} ({:.2}%)",
            io_busy,
            100.0 * (io_busy as f64 / clock_tick as f64)
        );
        println!("");
    }
}
