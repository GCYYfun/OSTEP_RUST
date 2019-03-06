extern crate rand;

mod process;
mod scheduler;
mod mlfq;
mod lottery;
mod relocation;
mod segmentation;
mod malloc;
mod paging_linear_translate;
mod paging_multilevel_translate;
mod paging_policy;

mod threadintro;

mod disk;
mod raid;

mod vsfs;
mod afs;


use std::io;




fn main() {
    start_os();
}

fn start_os() {

    println!("Welcome to discrete OS");
    
    let stdin = io::stdin();

    loop {

        let mut op = String::new();

        stdin.read_line(&mut op)
        .ok()
        .expect("Input Exception");

        print!("your input op is : {}",op);

        if op.trim() == "exit()"
        {
            break;
        }

       parsing_op(&op);

    
    }
}

fn parsing_op(op:&str) {
     let op_vec:Vec<&str> = op.trim().split(" ").collect();
     //let mut s = Scheduler::new(process::Process_Switch_Behavior::SWITCH_ON_IO,process::IO_Done_Behavior::IO_RUN_LATER,100);

     match op_vec[0] {
         "process_run" => process::process_run_op_parse(op_vec),
         "scheduler" => scheduler::scheduler_op_parse(op_vec),
         "mlfq" => mlfq::mlfq_op_parse(op_vec),
         "lottery" => lottery::lottery_op_parse(op_vec),
         "relocation" => relocation::relocation_op_parse(op_vec),
         "segmentation" => segmentation::segmentation_op_parse(op_vec),
         "malloc" => malloc::malloc_op_parse(op_vec),
         "plt" => paging_linear_translate::plt_op_parse(op_vec),
         "pmt" => paging_multilevel_translate::pmt_op_parse(op_vec),
         "pp" => paging_policy::pp_op_parse(op_vec),
         "intro_x86" => threadintro::x86::x86_op_parse(op_vec),
         //"lock_x86" => threadintro::x86::x86_op_parse(op_vec),
         //  "disk" => 
         "raid" => raid::raid_op_parse(op_vec),
         "vsfs" => vsfs::vsfs_op_parse(op_vec),
         "afs" => afs::afs_op_parse(op_vec),
         _ => println!("I dont know,what are you  talking about"),
     }
}