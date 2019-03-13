// not done


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

fn FindQueue() {
 
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

    let quantum = HashMap::new();
}