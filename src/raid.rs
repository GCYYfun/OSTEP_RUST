use rand::prelude::*;
fn convert (size:String) -> i32 {
    let length = size.len();
    let lastchar = &size[length-1..length];
    let mut nsize = 0;
    if  lastchar == "k" || lastchar == "K" {
        let m = 1024;
        nsize = &size[0..length-1].parse().unwrap() * m;
    } else if lastchar == "m" || lastchar == "M" {
        let m = 1024*1024;
        nsize = &size[0..length-1].parse().unwrap() * m;
    }else if lastchar == "g" || lastchar == "G" {
        let m = 1024*1024*1024;
        nsize = &size[0..length-1].parse().unwrap() * m;
    } else {
        nsize =  size.parse().unwrap();
    }
    nsize
}

struct disk {

}

struct raid {
    
}






struct RaidOption {
    seed:i32,
    numDisks:i32,
    chunkSize:String,
    numRequests:i32,
    size:String,
    workload:String,
    writeFrac:i32,
    range:i32,
    level:i32,
    raid5type:String,
    reverse:bool,
    timing:bool,
    solve:bool,
}

impl RaidOption {
    fn new() -> RaidOption {
        RaidOption {
            seed:0,
            numDisks:4,
            chunkSize:String::from("4k"),
            numRequests:10,
            size:String::from("4k"),
            workload:String::from("rand"),
            writeFrac:0,
            range:10000,
            level:0,
            raid5type:String::from("LS"),
            reverse:false,
            timing:false,
            solve:false,
        }
    }
}

pub fn raid_op_parse(op_vec:Vec<&str>) {
        let mut raid_op = RaidOption::new();
        let mut i =1;
        while i<op_vec.len() {
            match op_vec[i] {
                "-s" =>{raid_op.seed = op_vec[i+1].parse().unwrap();i = i+2;},
                "-D" =>{raid_op.numDisks = op_vec[i+1].parse().unwrap();i = i+2;},
                "-C" =>{raid_op.chunkSize = op_vec[i+1].to_string();i = i+2;},
                "-n" =>{raid_op.numRequests = op_vec[i+1].parse().unwrap();i = i+2;},
                "-S" =>{raid_op.size = op_vec[i+1].to_string();i=i+2;},
                "-W" =>{raid_op.workload = op_vec[i+1].to_string();i=i+2;},
                "-w" =>{raid_op.writeFrac = op_vec[i+1].parse().unwrap();i=i+2;},
                "-R" =>{raid_op.range = op_vec[i+1].parse().unwrap();i = i+2;},
                "-L" =>{raid_op.level = op_vec[i+1].parse().unwrap();i = i+2;},
                "-5" =>{raid_op.raid5type = op_vec[i+1].to_string();i=i+2;},
                "-r" =>{raid_op.reverse = true;i=i+1;},
                "-t" =>{raid_op.timing = true;i=i+1;},
                "-c" =>{raid_op.solve = true;i=i+1;},
                _ => println!("raid_op_parse match wrong!!"),
            }
        }
        execute_raid_op(raid_op);
}

fn execute_raid_op(options:RaidOption) {

    let seed_u8 = options.seed as u8;
    let seed = [seed_u8+1,seed_u8+2,seed_u8+3,seed_u8+4,
                                    seed_u8+5,seed_u8+6,seed_u8+7,seed_u8+8,
                                    seed_u8+9,seed_u8+10,seed_u8+11,seed_u8+12,
                                    seed_u8+13,seed_u8+14,seed_u8+15,seed_u8+16];

    let mut rng = SmallRng::from_seed(seed);
}