// not done

use std::collections::HashMap;
use rand::prelude::*;
struct MlfqOption {
    seed:i32,
    numQueues:i32,
    quantum:i32,
    quantumList:String,
    numJobs:i32,
    maxlen:i32,
    maxio:i32,
    boost:i32,
    ioTime:i32,
    stay:bool,
    iobump:bool,
    jlist:String,
    solve:bool,
}

impl MlfqOption {
    fn new() -> MlfqOption {
        MlfqOption {
            seed:0,
            numQueues:3,
            quantum:10,
            quantumList:String::from(""),
            numJobs:3,
            maxlen:100,
            maxio:10,
            boost:0,
            ioTime:5,
            stay:false,
            iobump:false,
            jlist:String::from(""),
            solve:false,
        }
    }
}

fn FindQueue(queue:&HashMap<i32,Vec<i32>>,hiQueue:i32) -> i32{
    let mut q = hiQueue;
    while q > 0{
        if queue[&q].len() > 0 {
             return q
        }
        q -= 1;
    }

    if queue[&0].len() > 0{
        return 0;
    }
    return -1
}

pub fn mlfq_op_parse(op_vec:Vec<&str>) {
    let mut mlfq_op = MlfqOption::new();
    let mut i =1;
    while i<op_vec.len() {
        match op_vec[i] {
            "-s" =>{mlfq_op.seed = op_vec[i+1].parse().unwrap();i= i+2;},
            "-n" =>{mlfq_op.numQueues = op_vec[i+1].parse().unwrap();i = i+2;},
            "-q" =>{mlfq_op.quantum = op_vec[i+1].parse().unwrap();i=i+2;},
            "-Q" =>{mlfq_op.quantumList = op_vec[i+1].to_string();i=i+2;},
            "-j" =>{mlfq_op.numJobs = op_vec[i+1].parse().unwrap();i=i+2;},
            "-m" =>{mlfq_op.maxlen = op_vec[i+1].parse().unwrap();i= i+2;},
            "-M" =>{mlfq_op.maxio =op_vec[i+1].parse().unwrap();i= i+2;},
            "-B" =>{mlfq_op.boost = op_vec[i+1].parse().unwrap();i= i+2;},
            "-i" =>{mlfq_op.ioTime = op_vec[i+1].parse().unwrap();i = i+2;},
            "-S" =>{mlfq_op.stay = true;i=i+1;},
            "-I" =>{mlfq_op.iobump = true;i=i+1;},
            "-l" =>{mlfq_op.jlist = op_vec[i+1].to_string();i=i+2;},
            "-c" =>{mlfq_op.solve = true;i=i+1;},
            _ => println!("mlfq_op_parse match wrong!!"),
        }
    }
    execute_mlfq_op(mlfq_op);
}

fn execute_mlfq_op(options:MlfqOption) {

    let seed_u8 = options.seed as u8;
    let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

    let mut rng = SmallRng::from_seed(seed);

    let numQueues = options.numQueues;

    let mut quantum:HashMap<i32,i32> = HashMap::new();

    if options.quantumList != "" {
        let quantumLengths:Vec<&str> = options.quantumList.split(",").collect();
        let numQueues = quantumLengths.len();
        let mut qc = numQueues - 1;
        for i in 0..numQueues {
            quantum.insert(qc as i32, quantumLengths[i].parse().unwrap());
            qc -= 1;
        }
    }else {
        for i in 0..numQueues{
            quantum.insert(i,options.quantum);
        }
    }

    let hiQueue = numQueues - 1;

    let ioTime = options.ioTime;

    let mut ioDone:HashMap<i32,Vec<(i32,String)>> = HashMap::new();

    let mut job:HashMap<i32,HashMap<String,i32>> = HashMap::new();

    let mut jobCnt = 0;

    if options.jlist != "" {
        let allJobs:Vec<&str> = options.jlist.split(":").collect();
        for j in allJobs {
            let jobInfo:Vec<&str> = j.split(",").collect();
            if jobInfo.len() != 3 {
                //  not good;
            }

            assert!(jobInfo.len() == 3);

            let startTime:i32 = jobInfo[0].parse().unwrap();
            let runTime:i32 = jobInfo[1].parse().unwrap();
            let ioFreq:i32 = jobInfo[2].parse().unwrap();

            
            let mut jobContent:HashMap<String,i32> = HashMap::new();
            jobContent.insert("currPri".to_string(),hiQueue);
            jobContent.insert("ticksLeft".to_string(),*quantum.get(&hiQueue).unwrap());
            jobContent.insert("startTime".to_string(),startTime);
            jobContent.insert("runTime".to_string(),runTime);
            jobContent.insert("timeLeft".to_string(),runTime);
            jobContent.insert("ioFreq".to_string(),ioFreq);
            jobContent.insert("doingIO".to_string(),0);
            jobContent.insert("firstRun".to_string(),-1);
            // {'currPri':hiQueue, 'ticksLeft':quantum[hiQueue], 'startTime':startTime,
            //            'runTime':runTime, 'timeLeft':runTime, 'ioFreq':ioFreq, 'doingIO':False,
            //            'firstRun':-1}

            job.insert(jobCnt,jobContent);
            ioDone.entry(startTime).or_insert(Vec::new());
            ioDone.get_mut(&startTime).unwrap().push((jobCnt,"JOB BEGINS".to_string()));
            jobCnt += 1;
        }
    }else {
        for j in 0..options.numJobs {
            let mut startTime = 0;
            let rand_x:f64 = rand::thread_rng().gen();
            let mut runTime = (rand_x * options.maxlen as f64) as i32;
            let rand_y:f64 = rand::thread_rng().gen();
            let mut ioFreq = (rand_y * options.maxio as f64) as i32;

            let mut jobContent:HashMap<String,i32> = HashMap::new();
            jobContent.insert("currPri".to_string(),hiQueue);
            jobContent.insert("ticksLeft".to_string(),*quantum.get(&hiQueue).unwrap());
            jobContent.insert("startTime".to_string(),startTime);
            jobContent.insert("runTime".to_string(),runTime);
            jobContent.insert("timeLeft".to_string(),runTime);
            jobContent.insert("ioFreq".to_string(),ioFreq);
            jobContent.insert("doingIO".to_string(),0);
            jobContent.insert("firstRun".to_string(),-1);

            job.insert(jobCnt,jobContent);

            ioDone.entry(startTime).or_insert(Vec::new());
            ioDone.get_mut(&startTime).unwrap().push((jobCnt,"JOB BEGINS".to_string()));
            jobCnt += 1;
        }
    }

    let numJobs = job.len();

    println! ("Here is the list of inputs:");
    println! ("OPTIONS jobs {}",            numJobs);
    println! ("OPTIONS queues {}",          numQueues);
    for i in quantum.len()-1..0{
        println! ("OPTIONS quantum length for queue {} is {}" ,i, quantum[&(i as i32)]);
    }
        
    println! ("OPTIONS boost {}",           options.boost);
    println! ("OPTIONS ioTime {}",          options.ioTime);
    println! ("OPTIONS stayAfterIO {}",     options.stay);
    println! ("OPTIONS iobump {}",          options.iobump);

    println! ("");
    println! ("For each job, three defining characteristics are given:");
    println! ("  startTime : at what time does the job enter the system");
    println! ("  runTime   : the total CPU time needed by the job to finish");
    println! ("  ioFreq    : every ioFreq time units, the job issues an I/O");
    println! ("              (the I/O takes ioTime units to complete)");

    println! ("Job List:");
    for i in 0..numJobs{
        println! ("  Job {}: startTime {} - runTime {} - ioFreq {}" ,i, job[&(i as i32)]["startTime"],
                                                                    job[&(i as i32)]["runTime"], job[&(i as i32)]["ioFreq"]);
    }
        
    println! ("");

    if options.solve == false{
        println! ("Compute the execution trace for the given workloads.");
        println! ("If you would like, also compute the response and turnaround");
        println! ("times for each of the jobs.");
        println! ("");
        println! ("Use the -c flag to get the exact results when you are finished.");
    }


    let mut queue:HashMap<i32,Vec<i32>> = HashMap::new();

    for q in 0..numQueues {
        queue.entry(q).or_insert(Vec::new());
    }

    let mut currTime = 0;

    let totalJobs = job.len();

    let mut finishedJobs = 0;
    println!("");
    println!("Execution Trace:");

    while finishedJobs < totalJobs {
        if options.boost > 0 && currTime != 0 {
            if currTime % options.boost == 0 {
                println! ("[ time {} ] BOOST ( every {} )" , currTime, options.boost);

                for q in 0..numQueues-1 {
                    let vs = queue.get_mut(&(q as i32)).unwrap().clone();
                    for j in vs {
                        if job[&(j as i32)]["doingIO"] == 0 {
                            queue.get_mut(&hiQueue).unwrap().push(j as i32);
                        }
                    }
                    queue.entry(q).or_insert(Vec::new());
                }

                for j in 0..numJobs {
                    if job[&(j as i32)]["timeLeft"] > 0 {
                        job.get_mut(&(j as i32)).unwrap().insert("currPri".to_string(),hiQueue);
                        job.get_mut(&(j as i32)).unwrap().insert("ticksLeft".to_string(),quantum[&hiQueue]);
                        // job[&(j as i32)]["currPri"]   = hiQueue
                        // job[&(j as i32)]["ticksLeft"] = quantum[&hiQueue]
                    }
                }
            }
        }

        if ioDone.contains_key(&currTime) {
            for t in &ioDone[&currTime]{
                let j = t.0;
                let typei:String = t.1.clone();
                let q = job[&j]["currPri"];
                job.get_mut(&(j as i32)).unwrap().insert("doingIO".to_string(),0);
                println! ("[ time {} ] {} by JOB {}" ,currTime, typei, j);

                if options.iobump == false{
                    queue.get_mut(&q).unwrap().push(j);
                }else {
                    queue.get_mut(&q).unwrap().insert(0, j);
                }
            }
            
        }

        let currQueue = FindQueue(&queue,hiQueue);

        if currQueue == -1{
            println!("[ time {} ] IDLE" , currTime);
            currTime += 1;
            continue;
        }

        let currJob = queue[&currQueue][0];

        *job.get_mut(&currJob).unwrap().get_mut("timeLeft").unwrap() -= 1;
        *job.get_mut(&currJob).unwrap().get_mut("ticksLeft").unwrap()  -= 1;

        if job[&currJob]["firstRun"] == -1{
            *job.get_mut(&currJob).unwrap().get_mut("firstRun").unwrap()  = currTime;
        }

        let mut runTime   = job[&currJob]["runTime"];
        let mut ioFreq    = job[&currJob]["ioFreq"];
        let mut ticksLeft = job[&currJob]["ticksLeft"];
        let mut timeLeft  = job[&currJob]["timeLeft"];

        println! ("[ time {} ] Run JOB {} at PRIORITY {} [ TICKSLEFT {} RUNTIME {} TIMELEFT {} ]" , currTime, currJob, currQueue, ticksLeft, runTime, timeLeft);
            
        currTime += 1;

        if timeLeft == 0 {
            println!("[ time {} ] FINISHED JOB {}" , currTime,currJob);
            finishedJobs += 1;

            job.get_mut(&currJob).unwrap().insert("endTime".to_string(),currTime);
            let done = queue[&currQueue].remove(0);
            
            assert!(done == currJob);
            continue;
        }

        let mut issuedIO = false;
    }
        
}