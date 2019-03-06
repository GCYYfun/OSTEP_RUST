use std::collections::HashMap;
use rand::prelude::*;



//
//  Which files are used in the simulation
//  
//  Not representing a realistic piece of anything
//  but rather just for convenience when generating
//  random traces ...
// 
//  Files are named 'a', 'b', etc. for ease of use
//  Could probably add a numeric aspect to allow
//  for more than 26 files but who cares
// 
struct Files {
    numfiles:i32,
    value:i32,
    filelist:Vec<String>,
}

impl Files {
    fn new(numfiles:i32) -> Files {
        Files {
            numfiles:numfiles,
            value:0,
            filelist:vec!["a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"e".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),"i".to_string(),"j".to_string(),"k".to_string(),"l".to_string(),"m".to_string(),"n".to_string(),"o".to_string(),"p".to_string(),"q".to_string(),"r".to_string(),"s".to_string(),"t".to_string(),"u".to_string(),"v".to_string(),"w".to_string(),"x".to_string(),"y".to_string(),"z".to_string()],
        }
    }

    fn init(&mut self) {
        self.filelist = self.filelist[0..self.numfiles as usize].to_vec();
    }

    fn getfiles(&self) ->Vec<String>{
        return self.filelist.clone();
    }

    fn getvalue(&mut self) -> i32 {
        let rc = self.value;
        self.value += 1;
        return rc;
    }
}




struct AfsOption {
    seed:i32,
    numclients:i32,
    numsteps:i32,
    numfiles:i32,
    readratio:f64,
    actions:String,
    schedule:String,
    printstats:bool,
    solve:bool,
    detail:i32,
}

impl AfsOption {
    pub fn new() -> AfsOption {
        AfsOption {
            seed:0,
            numclients:2,
            numsteps:2,
            numfiles:1,
            readratio:0.5,
            actions:String::from(""),
            schedule:String::from(""),
            printstats:false,
            solve:false,
            detail:0,
        }
    }
}

pub fn afs_op_parse(op_vec:Vec<&str>) {
    let mut afs_op = AfsOption::new();
    let mut i =1;
    while i<op_vec.len() {
        match op_vec[i] {
            "-s" =>{afs_op.seed = op_vec[i+1].parse().unwrap();i = i+2;},
            "-C" =>{afs_op.numclients = op_vec[i+1].parse().unwrap();i = i+2;},
            "-n" =>{afs_op.numsteps = op_vec[i+1].parse().unwrap();i = i+2;},
            "-f" =>{afs_op.numfiles = op_vec[i+1].parse().unwrap();i=i+2;},
            "-r" =>{afs_op.readratio = op_vec[i+1].parse().unwrap();i=i+2;},
            "-A" =>{afs_op.actions = op_vec[i+1].to_string();i=i+2;},
            "-S" =>{afs_op.schedule = op_vec[i+1].to_string();i=i+2;},
            "-p" =>{afs_op.printstats = true;i=i+1;},
            "-c" =>{afs_op.solve = true;i=i+1;},
            "-d" =>{afs_op.detail = op_vec[i+1].parse().unwrap();i=i+2;},
            _ => println!("afs_op_parse match wrong!!"),
        }
    }
    execute_afs_op(afs_op);
}

fn execute_afs_op(options:AfsOption) {

    let seed_u8 = options.seed as u8;
    let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

    let mut rng = SmallRng::from_seed(seed);
}