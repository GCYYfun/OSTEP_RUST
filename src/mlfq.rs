// not done


struct mlfq_option {
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

impl mlfq_option {
    fn new() -> mlfq_option {
        mlfq_option {
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

}