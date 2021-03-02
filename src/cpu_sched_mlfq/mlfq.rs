// not done

use rand::{Rng, SeedableRng};
use std::collections::HashMap;

const HELP: &str = include_str!("help.txt");

struct MlfqOption {
    seed: u64,
    num_queues: i32,
    quantum: i32,
    quantum_list: String,
    allotment: i32,
    allotment_list: String,
    num_jobs: i32,
    maxlen: i32,
    maxio: i32,
    boost: i32,
    io_time: i32,
    stay: bool,
    iobump: bool,
    jlist: String,
    solve: bool,
}

impl MlfqOption {
    fn new() -> MlfqOption {
        MlfqOption {
            seed: 0,
            num_queues: 3,
            quantum: 10,
            quantum_list: String::from(""),
            allotment: 1,
            allotment_list: String::from(""),
            num_jobs: 3,
            maxlen: 100,
            maxio: 10,
            boost: 0,
            io_time: 5,
            stay: false,
            iobump: false,
            jlist: String::from(""),
            solve: false,
        }
    }
}

fn find_queue(queue: &HashMap<i32, Vec<i32>>, hi_queue: i32) -> i32 {
    let mut q = hi_queue;
    while q > 0 {
        if queue[&q].len() > 0 {
            return q;
        }
        q -= 1;
    }

    if queue[&0].len() > 0 {
        return 0;
    }
    return -1;
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut mlfq_op = MlfqOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" => {
                mlfq_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-n" => {
                mlfq_op.num_queues = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-q" => {
                mlfq_op.quantum = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-Q" => {
                mlfq_op.quantum_list = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-a" => {
                mlfq_op.allotment = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-A" => {
                mlfq_op.allotment_list = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-j" => {
                mlfq_op.num_jobs = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-m" => {
                mlfq_op.maxlen = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-M" => {
                mlfq_op.maxio = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-B" => {
                mlfq_op.boost = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-i" => {
                mlfq_op.io_time = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-S" => {
                mlfq_op.stay = true;
                i = i + 1;
            }
            "-I" => {
                mlfq_op.iobump = true;
                i = i + 1;
            }
            "-l" => {
                mlfq_op.jlist = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-c" => {
                mlfq_op.solve = true;
                i = i + 1;
            }
            _ => {
                println!("mlfq_op_parse match wrong!!");
                return;
            }
        }
    }
    execute_mlfq_op(mlfq_op);
}

fn execute_mlfq_op(options: MlfqOption) {
    let _rng = rand_chacha::ChaCha8Rng::seed_from_u64(options.seed);

    let mut num_queues = options.num_queues;

    let mut quantum: HashMap<i32, i32> = HashMap::new();

    if options.quantum_list != "" {
        let quantum_lengths: Vec<&str> = options.quantum_list.split(",").collect();
        num_queues = quantum_lengths.len() as i32;
        let mut qc = num_queues - 1;
        for i in 0..num_queues {
            quantum.insert(qc as i32, quantum_lengths[i as usize].parse().unwrap());
            qc -= 1;
        }
    } else {
        for i in 0..num_queues {
            quantum.insert(i, options.quantum);
        }
    }

    let mut allotment: HashMap<i32, i32> = HashMap::new();

    if options.allotment_list != "" {
        let allotment_lengths: Vec<&str> = options.quantum_list.split(",").collect();
        if num_queues != allotment_lengths.len() as i32 {
            println!("number of allotments specified must match number of quantums");
            return;
        }
        let mut qc = num_queues - 1;
        for i in 0..num_queues {
            allotment.insert(qc as i32, allotment_lengths[i as usize].parse().unwrap());
            if qc != 0 && allotment[&qc] <= 0 {
                println!("allotment must be positive integer");
                return;
            }
            qc -= 1;
        }
    } else {
        for i in 0..num_queues {
            allotment.insert(i, options.allotment);
        }
    }

    let hi_queue = num_queues - 1;

    let io_time = options.io_time;

    let mut io_done: HashMap<i32, Vec<(i32, String)>> = HashMap::new();

    let mut job: HashMap<i32, HashMap<String, i32>> = HashMap::new();

    let mut job_cnt = 0;

    if options.jlist != "" {
        let all_jobs: Vec<&str> = options.jlist.split(":").collect();
        for j in all_jobs {
            let job_info: Vec<&str> = j.split(",").collect();
            if job_info.len() != 3 {
                //  not good;
                return;
            }

            assert!(job_info.len() == 3);

            let start_time: i32 = job_info[0].parse().unwrap();
            let run_time: i32 = job_info[1].parse().unwrap();
            let io_freq: i32 = job_info[2].parse().unwrap();

            let mut job_content: HashMap<String, i32> = HashMap::new();
            job_content.insert("currPri".to_string(), hi_queue);
            job_content.insert("ticksLeft".to_string(), *quantum.get(&hi_queue).unwrap());
            job_content.insert("allotLeft".to_string(), *allotment.get(&hi_queue).unwrap());
            job_content.insert("start_time".to_string(), start_time);
            job_content.insert("run_time".to_string(), run_time);
            job_content.insert("timeLeft".to_string(), run_time);
            job_content.insert("io_freq".to_string(), io_freq);
            job_content.insert("doingIO".to_string(), 0);
            job_content.insert("firstRun".to_string(), -1);
            // {'currPri':hi_queue, 'ticksLeft':quantum[hi_queue], 'start_time':start_time,
            //            'run_time':run_time, 'timeLeft':run_time, 'io_freq':io_freq, 'doingIO':False,
            //            'firstRun':-1}

            job.insert(job_cnt, job_content);
            io_done.entry(start_time).or_insert(vec![]);
            io_done
                .get_mut(&start_time)
                .unwrap()
                .push((job_cnt, "JOB BEGINS".to_string()));
            job_cnt += 1;
        }
    } else {
        for _j in 0..options.num_jobs {
            let start_time = 0;
            let rand_x: f64 = rand::thread_rng().gen();
            let run_time = (rand_x * options.maxlen as f64) as i32;
            let rand_y: f64 = rand::thread_rng().gen();
            let io_freq = (rand_y * options.maxio as f64) as i32;

            let mut job_content: HashMap<String, i32> = HashMap::new();
            job_content.insert("currPri".to_string(), hi_queue);
            job_content.insert("ticksLeft".to_string(), *quantum.get(&hi_queue).unwrap());
            job_content.insert("allotLeft".to_string(), *allotment.get(&hi_queue).unwrap());
            job_content.insert("start_time".to_string(), start_time);
            job_content.insert("run_time".to_string(), run_time);
            job_content.insert("timeLeft".to_string(), run_time);
            job_content.insert("io_freq".to_string(), io_freq);
            job_content.insert("doingIO".to_string(), 0);
            job_content.insert("firstRun".to_string(), -1);

            job.insert(job_cnt, job_content);
            io_done.entry(start_time).or_insert(vec![]);
            io_done
                .get_mut(&start_time)
                .unwrap()
                .push((job_cnt, "JOB BEGINS".to_string()));
            job_cnt += 1;
        }
    }

    let num_jobs = job.len();

    println!("Here is the list of inputs:");
    println!("OPTIONS jobs {}", num_jobs);
    println!("OPTIONS queues {}", num_queues);
    for i in (0..quantum.len()).rev() {
        println!(
            "OPTIONS allotments for queue {:2} is {:3}",
            i,
            allotment[&(i as i32)]
        );
        println!(
            "OPTIONS quantum length for queue {:2} is {:3}",
            i,
            quantum[&(i as i32)]
        );
    }

    println!("OPTIONS boost {}", options.boost);
    println!("OPTIONS io_time {}", options.io_time);
    println!("OPTIONS stayAfterIO {}", options.stay);
    println!("OPTIONS iobump {}", options.iobump);

    println!("");
    println!("For each job, three defining characteristics are given:");
    println!("  start_time : at what time does the job enter the system");
    println!("  run_time   : the total CPU time needed by the job to finish");
    println!("  io_freq    : every io_freq time units, the job issues an I/O");
    println!("              (the I/O takes io_time units to complete)");

    println!("Job List:");
    for i in 0..num_jobs {
        println!(
            "  Job {:2}: start_time {:3} - run_time {:3} - io_freq {:3}",
            i,
            job[&(i as i32)]["start_time"],
            job[&(i as i32)]["run_time"],
            job[&(i as i32)]["io_freq"]
        );
    }

    println!("");

    if options.solve == false {
        println!("Compute the execution trace for the given workloads.");
        println!("If you would like, also compute the response and turnaround");
        println!("times for each of the jobs.");
        println!("");
        println!("Use the -c flag to get the exact results when you are finished.");
        return;
    }

    let mut queue: HashMap<i32, Vec<i32>> = HashMap::new();

    for q in 0..num_queues {
        queue.entry(q).or_insert(vec![]);
    }

    let mut curr_time = 0;

    let total_jobs = job.len();

    let mut finished_jobs = 0;
    println!("");
    println!("Execution Trace:");

    while finished_jobs < total_jobs {
        if options.boost > 0 && curr_time != 0 {
            if curr_time % options.boost == 0 {
                println!("[ time {} ] BOOST ( every {} )", curr_time, options.boost);

                for q in 0..num_queues - 1 {
                    let vs = queue.get_mut(&(q as i32)).unwrap().clone();
                    for j in vs {
                        if job[&(j as i32)]["doingIO"] == 0 {
                            queue.get_mut(&hi_queue).unwrap().push(j as i32);
                        }
                    }
                    queue.entry(q).or_insert(Vec::new());
                }

                for j in 0..num_jobs {
                    if job[&(j as i32)]["timeLeft"] > 0 {
                        job.get_mut(&(j as i32))
                            .unwrap()
                            .insert("currPri".to_string(), hi_queue);
                        job.get_mut(&(j as i32))
                            .unwrap()
                            .insert("ticksLeft".to_string(), quantum[&hi_queue]);
                        // job[&(j as i32)]["currPri"]   = hi_queue
                        // job[&(j as i32)]["ticksLeft"] = quantum[&hi_queue]
                    }
                }
            }
        }

        if io_done.contains_key(&curr_time) {
            for t in &io_done[&curr_time] {
                let j = t.0;
                let typei: String = t.1.clone();
                let q = job[&j]["currPri"];
                job.get_mut(&(j as i32))
                    .unwrap()
                    .insert("doingIO".to_string(), 0);
                println!("[ time {} ] {} by JOB {}", curr_time, typei, j);

                if options.iobump == false || typei == "JOB BEGINS" {
                    queue.get_mut(&q).unwrap().push(j);
                } else {
                    queue.get_mut(&q).unwrap().insert(0, j);
                }
            }
        }

        let curr_queue = find_queue(&queue, hi_queue);

        if curr_queue == -1 {
            println!("[ time {} ] IDLE", curr_time);
            curr_time += 1;
            continue;
        }

        let curr_job = queue[&curr_queue][0];

        if job[&curr_job]["currPri"] != curr_queue {
            panic!(
                "currPri[{}] does not match currQueue[{}]",
                job[&curr_job]["currPri"], curr_queue
            );
        }

        *job.get_mut(&curr_job).unwrap().get_mut("timeLeft").unwrap() -= 1;
        *job.get_mut(&curr_job)
            .unwrap()
            .get_mut("ticksLeft")
            .unwrap() -= 1;

        if job[&curr_job]["firstRun"] == -1 {
            *job.get_mut(&curr_job).unwrap().get_mut("firstRun").unwrap() = curr_time;
        }

        let run_time = job[&curr_job]["run_time"];
        let io_freq = job[&curr_job]["io_freq"];
        let ticks_left = job[&curr_job]["ticksLeft"];
        let allot_left = job[&curr_job]["allotLeft"];
        let time_left = job[&curr_job]["timeLeft"];

        println!(
            "[ time {} ] Run JOB {} at PRIORITY {} [ TICKS {} ALLOT {} TIME {} (of {}) ]",
            curr_time, curr_job, curr_queue, ticks_left, allot_left, time_left, run_time
        );

        if time_left < 0 {
            panic!("Error: should never have less than 0 time left to run");
        }

        curr_time += 1;

        if time_left == 0 {
            println!("[ time {} ] FINISHED JOB {}", curr_time, curr_job);
            finished_jobs += 1;

            job.get_mut(&curr_job)
                .unwrap()
                .insert("endTime".to_string(), curr_time);

            let done = queue.get_mut(&curr_queue).unwrap().remove(0);

            assert!(done == curr_job);
            continue;
        }

        let mut issued_io = false;
        if io_freq > 0 && ((run_time - time_left) % io_freq) == 0 {
            println!("[ time {} ] IO_START by JOB {}", curr_time, curr_job);

            issued_io = true;

            let _desched = queue.get_mut(&curr_queue).unwrap().remove(0);

            job.get_mut(&curr_job)
                .unwrap()
                .insert("doingIO".to_string(), 1);

            if options.stay == true {
                job.get_mut(&curr_job)
                    .unwrap()
                    .insert("ticksLeft".to_string(), quantum[&curr_queue]);
                job.get_mut(&curr_job)
                    .unwrap()
                    .insert("allotLeft".to_string(), allotment[&curr_queue]);
            }

            let future_time = curr_time + io_time;

            io_done.entry(future_time).or_insert(vec![]);

            println!("IO DONE");

            io_done
                .get_mut(&future_time)
                .unwrap()
                .push((curr_job, "IO_DONE".to_string()))
        }

        if ticks_left == 0 {
            let mut _desched: i32 = -999;
            if issued_io == false {
                _desched = queue.get_mut(&curr_queue).unwrap().remove(0);
            }
            // assert!(&desched == &curr_job);
            let n = job[&curr_job]["allotLeft"] - 1;
            job.get_mut(&curr_job)
                .unwrap()
                .insert("allotLeft".to_string(), n);

            if job[&curr_job]["allotLeft"] == 0 {
                if curr_queue > 0 {
                    job.get_mut(&curr_job)
                        .unwrap()
                        .insert("currPri".to_string(), curr_queue - 1);
                    job.get_mut(&curr_job)
                        .unwrap()
                        .insert("ticksLeft".to_string(), quantum[&(curr_queue - 1)]);
                    job.get_mut(&curr_job)
                        .unwrap()
                        .insert("allotLeft".to_string(), curr_queue - 1);
                    if issued_io == false {
                        queue.get_mut(&(curr_queue - 1)).unwrap().push(curr_job);
                    }
                } else {
                    if issued_io == false {
                        job.get_mut(&curr_job)
                            .unwrap()
                            .insert("ticksLeft".to_string(), quantum[&curr_queue]);
                        job.get_mut(&curr_job)
                            .unwrap()
                            .insert("allotLeft".to_string(), allotment[&curr_queue]);
                        queue.get_mut(&curr_queue).unwrap().push(curr_job);
                    }
                }
            } else {
                job.get_mut(&curr_job)
                    .unwrap()
                    .insert("ticksLeft".to_string(), quantum[&curr_queue]);
                if issued_io == false {
                    queue.get_mut(&curr_queue).unwrap().push(curr_job);
                }
            }
        }
    }

    println!();
    println!("Final statistics:");
    let mut response_sum = 0;
    let mut turnaround_sum = 0;
    for i in 0..num_jobs {
        let response = job[&(i as i32)]["firstRun"] - job[&(i as i32)]["start_time"];
        let turnaround = job[&(i as i32)]["endTime"] - job[&(i as i32)]["start_time"];

        println!(
            "  Job {:2}: startTime {:3} - response {:3} - turnaround {:3}",
            i,
            job[&(i as i32)]["start_time"],
            response,
            turnaround
        );

        response_sum = response + response_sum;
        turnaround_sum = turnaround + turnaround_sum;
    }
    println!();
    println!(
        "  {:2} Job Avg : startTime n/a - response {:.2} - turnaround {:.2}",
        num_jobs,
        response_sum as f64 / num_jobs as f64,
        turnaround_sum as f64 / num_jobs as f64
    );
    println!();
}
