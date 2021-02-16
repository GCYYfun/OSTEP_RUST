//  done

use rand::{Rng, SeedableRng};

const HELP: &str = include_str!("help.txt");

struct LotteryOption {
    seed: u64,
    jobs: i32,
    jlist: String,
    maxlen: i32,
    maxticket: i32,
    quantum: i32,
    solve: bool,
}

impl LotteryOption {
    pub fn new() -> LotteryOption {
        LotteryOption {
            seed: 0,
            jobs: 3,
            jlist: String::from(""),
            maxlen: 10,
            maxticket: 100,
            quantum: 1,
            solve: false,
        }
    }
}

pub fn parse_op(op_vec: Vec<&str>) {
    let mut lott_op = LotteryOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            "-s" => {
                lott_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-j" => {
                lott_op.jobs = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-l" => {
                lott_op.jlist = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-m" => {
                lott_op.maxlen = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-p" => {
                lott_op.maxticket = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-q" => {
                lott_op.quantum = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-c" => {
                lott_op.solve = true;
                i = i + 1;
            }
            _ => {
                println!("lottery_op_parse match wrong!!");
                return;
            }
        }
    }
    execute_lottery_op(lott_op);
}

fn execute_lottery_op(options: LotteryOption) {
    // let seed_u8 = options.seed as u8;
    // let seed = [
    //     seed_u8 + 1,
    //     seed_u8 + 2,
    //     seed_u8 + 3,
    //     seed_u8 + 4,
    //     seed_u8 + 5,
    //     seed_u8 + 6,
    //     seed_u8 + 7,
    //     seed_u8 + 8,
    //     seed_u8 + 9,
    //     seed_u8 + 10,
    //     seed_u8 + 11,
    //     seed_u8 + 12,
    //     seed_u8 + 13,
    //     seed_u8 + 14,
    //     seed_u8 + 15,
    //     seed_u8 + 16,
    // ];

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(options.seed);

    println!("ARG jlist : {}", options.jlist);
    println!("ARG jobs : {}", options.jobs);
    println!("ARG maxlen {} ", options.maxlen);
    println!("ARG maxticket : {} ", options.maxticket);
    println!("ARG quantum : {} ", options.quantum);
    println!("ARG seed : {} ", options.seed);
    println!("");
    println!("Here is the job list, with the run time of each job: ");

    let mut tick_total = 0;
    let mut run_total = 0;
    let mut joblist: Vec<Vec<f32>> = Vec::new();

    if options.jlist == "" {
        for jobnum in 0..options.jobs {
            let rand_x: f32 = rng.gen();
            let runtime = (options.maxlen as f32 * rand_x) as i32;
            let tickets = (options.maxticket as f32 * rand_x) as i32;
            run_total += runtime;
            tick_total += tickets;
            let mut v: Vec<f32> = Vec::new();
            v.push(jobnum as f32);
            v.push(runtime as f32);
            v.push(tickets as f32);
            joblist.push(v);
            println!(
                "  Job {} ( length = {}, tickets = {} )",
                jobnum, runtime, tickets
            );
        }
    } else {
        let mut _jobnum = 0;
        for entry in options.jlist.split(',') {
            let _runtime: f32;
            let _tickets: i32;
            let info: Vec<&str> = entry.split(':').collect();
            let runtime: i32 = info[0].parse().unwrap();
            let tickets: i32 = info[1].parse().unwrap();
            run_total += runtime;
            tick_total += tickets;
            _jobnum += 1;
        }
        for job in &joblist {
            println!(
                "  Job {} ( length = {}, tickets = {} )",
                job[0], job[1], job[2]
            );
        }
    }
    println!("");

    if options.solve == false {
        println!("Here is the set of random numbers you will need (at most):");
        for _i in 0..run_total {
            let rand_x: f32 = rng.gen();
            let r = (rand_x * 1000001.0) as i32;
            println!("Random {} ", r);
        }
    }

    if options.solve == true {
        println!("** Solutions **");

        let mut jobs = joblist.len();
        let mut clock = 0;
        for _i in 0..run_total {
            let rand_x: f32 = rng.gen();
            let r = (rand_x * 1000001.0) as i32;
            let winner = (r % tick_total) as i32;

            let mut wjob = 0;
            let mut wrun = 0.0;
            let mut wtix = 0;

            let mut current = 0;
            for v in &joblist {
                current += v[2] as i32;
                if current > winner {
                    wjob = v[0] as i32;
                    wrun = v[1];
                    wtix = v[2] as i32;
                    break;
                }
            }

            println!(
                "Random {} -> Winning ticket {} (of {}) -> Run {} ",
                r, winner, tick_total, wjob
            );

            println!("  Jobs:");
            for v in &joblist {
                let mut _wstr = "";
                if wjob == v[0] as i32 {
                    _wstr = "*";
                } else {
                    _wstr = " ";
                }

                let mut _tstr = String::from("");
                if v[1] > 0.0 {
                    _tstr = v[2].to_string();
                } else {
                    _tstr = "---".to_string();
                }
                println!(
                    "{} job : {} timeletft: {} tix: {} ",
                    _wstr, v[0], v[1], _tstr
                );
            }
            println!("");

            if wrun >= options.quantum as f32 {
                wrun -= options.quantum as f32;
            } else {
                wrun = 0.0;
            }

            clock += options.quantum;

            if wrun == 0.0 {
                println!("--> JOB {} DONE at time {}", wjob, clock);
                tick_total -= wtix;
                wtix = 0;
                jobs -= 1;
            }

            joblist[wjob as usize][0] = wjob as f32;
            joblist[wjob as usize][1] = wrun as f32;
            joblist[wjob as usize][2] = wtix as f32;

            if jobs == 0 {
                println!("");
                break;
            }
        }
    }
}
