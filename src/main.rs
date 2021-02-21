#![allow(dead_code)]
#![allow(mutable_borrow_reservation_conflict)] // 危险
use std::env;

/*
    Virtualization
*/
// CPU
// mod cpu_api;
mod cpu_intro;
mod cpu_sched;
mod cpu_sched_lottery;
mod cpu_sched_mlfq; // not done

// VM
mod vm_beyondphys_policy;
mod vm_freespace;
mod vm_mechanism;
mod vm_paging;
mod vm_segmentation;
mod vm_smalltables;

/*
    Concurrency
*/

// THREADS
// mod threads_intro;

/*
    Persistence
*/

// FILE
mod dist_afs;
mod file_implementation;
mod file_raid;


// util
mod help;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let args: Vec<&str> = args.iter().map(AsRef::as_ref).collect();

    if args.len() < 2 {
        println!("Nothing to do ,please use 'help' for some help ");
        return;
    }

    match args[1] {
        "help" => {
            help::help();
        }
        "process_run" | "pr" => {
            cpu_intro::parse_op(args);
        }
        // "fork" => {
        //     cpu_api::pares_op(args);
        // }
        "scheduler" => {
            cpu_sched::parse_op(args);
        }
        "lottery" => {
            cpu_sched_lottery::parse_op(args);
        }
        "mlfq" => {
            cpu_sched_mlfq::parse_op(args);
        }
        "relocation" => {
            vm_mechanism::parse_op(args);
        }
        "segmentation" => {
            vm_segmentation::parse_op(args);
        }
        "malloc" => {
            vm_freespace::parse_op(args);
        }
        "paging_linear_translate" | "plt" => {
            vm_paging::parse_op(args);
        }
        "paging_multilevel_translate" | "pmt" => {
            vm_smalltables::parse_op(args);
        }
        "paging_policy" | "pp" => {
            vm_beyondphys_policy::parse_op(args);
        }
        "vsfs" => {
            file_implementation::parse_op(args);
        }
        "raid" => {
            file_raid::parse_op(args);
        }
        "afs" => {
            dist_afs::parse_op(args);
        }
        _ => println!("I dont know,what are you talking about,please use '-h' for some help "),
    }
}
