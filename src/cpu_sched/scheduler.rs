// almost done except RR SJF

use rand::{Rng, SeedableRng};

const HELP: &str = include_str!("help.txt");

struct SchedulerOption {
    seed: u64,
    jobs: u32,
    jlist: String,
    maxlen: u32,
    policy: String,
    quantum: u32,
    solve: bool,
}

impl SchedulerOption {
    fn new() -> SchedulerOption {
        SchedulerOption {
            seed: 0,
            jobs: 3,
            jlist: String::from(""),
            maxlen: 10,
            policy: String::from("FIFO"),
            quantum: 1,
            solve: false,
        }
    }
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut sche_op = SchedulerOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        println!("{}", op_vec[i]);
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" => {
                sche_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-j" => {
                sche_op.jobs = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-l" => {
                sche_op.jlist = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-m" => {
                sche_op.maxlen = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-p" => {
                sche_op.policy = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-q" => {
                sche_op.quantum = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-c" => {
                sche_op.solve = true;
                i = i + 1;
            }
            _ => {
                println!("scheduler_op_parse match wrong!!");
                return;
            }
        }
    }
    execute_scheduler_op(sche_op);
}

fn execute_scheduler_op(mut options: SchedulerOption) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(options.seed);

    println!("ARG policy : {}", options.policy);
    if options.jlist == "" {
        println!("ARG jobs : {}", options.jobs);
        println!("ARG maxlen : {}", options.maxlen);
        println!("ARG seed : {}", options.seed);
    } else {
        println!("ARG jlist : {}", options.jlist)
    }

    println!("");

    println!("Here is the job list, with the run time of each job: ");

    let mut joblist: Vec<Vec<f32>> = Vec::new();
    if options.jlist == "" {
        for jobnum in 0..options.jobs {
            let rand_x: f64 = rng.gen();
            let runtime = (options.maxlen as f64 * rand_x) as u32 + 1;
            let mut v: Vec<f32> = Vec::new();
            v.push(jobnum as f32);
            v.push(runtime as f32);
            joblist.push(v);
            println!("  Job : {}  ( length = {} )", jobnum, runtime);
        }
    } else {
        let mut jobnum = 0;
        for runtime in options.jlist.split(",") {
            let mut v: Vec<f32> = Vec::new();
            v.push(jobnum as f32);
            v.push(runtime.to_string().parse::<f32>().unwrap());
            joblist.push(v);
            jobnum += 1;
        }
        for job in &joblist {
            println!("Job : {}  (length = {} )", job[0], job[1].to_string());
        }
    }

    println!("");

    if options.solve == true {
        println!("** Solutions **");
        if options.policy == "SJF" {
            // not impl SJF
            joblist.sort_by(|a, b| a.partial_cmp(b).unwrap());
            options.policy = "FIFO".to_string();
        }

        if options.policy == "FIFO" {
            let mut thetime: f32 = 0.0;
            println!("Execution trace:");
            for job in &joblist {
                println!("  [ time {time:>width$} ] Run job {job0} for {job1} secs ( DONE at {donetime} )" ,time = thetime,width = 3, job0 = job[0], job1= format!("{:.*}",2,job[1]),donetime = format!("{:.*}",2,(thetime + job[1])));
                thetime += job[1];
            }

            println!("Final statistics :");
            let mut t = 0.0;
            let mut count = 0;
            let mut turnaround_sum = 0.0;
            let mut wait_sum = 0.0;
            let mut response_sum = 0.0;
            for tmp in &joblist {
                let jobnum = tmp[0];
                let runtime = tmp[1];

                let response = t;
                let turnaround = t + runtime;
                let wait = t;
                println!(
                    "  Job {} -- Response: {}  Turnaround {} Wait {}",
                    jobnum, response, turnaround, wait
                );
                response_sum += response;
                turnaround_sum += turnaround;
                wait_sum += wait;
                t += runtime;
                count = count + 1;
            }
            println!(
                "  Average -- Response: {}  Turnaround : {}  Wait : {}",
                response_sum / count as f32,
                turnaround_sum / count as f32,
                wait_sum / count as f32
            );
        }

        if options.policy == "RR" {
            // not impl RR
            println!("Execution trace:");
        }

        if options.policy != "FIFO" && options.policy != "SJF" && options.policy != "RR" {
            println!("Error : Policy {} is not available", options.policy);
        }
    } else {
        println!("Compute the turnaround time, response time, and wait time for each job.");
        println!("When you are done, run this program again, with the same arguments,");
        println!("but with -c, which will thus provide you with the answers. You can use");
        println!("-s <somenumber> or your own job list (-l 10,15,20 for example)");
        println!("to generate different problems for yourself.");
        println!("");
    }
}
