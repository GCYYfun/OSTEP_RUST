use rand::{Rng, SeedableRng};
use std::collections::HashMap;

const HELP: &str = include_str!("help.txt");

struct Forker {
    fork_percentage:f64,
    max_action:u32,
    action_list:Vec<String>,
    show_tree:bool,
    just_final:bool,
    leaf_only:bool,
    local_reparent:bool,
    print_style:bool,
    solve:bool,
    root_name:String,
    process_list:Vec<String>,
    children:HashMap<String,Vec<String>>,
    parents:HashMap<String,Vec<String>>,
    name_length:u32,
    base_names:String,
    curr_names:String,
    curr_index:usize,
}

impl Forker {
    fn new(fork_percentage:f64,
        max_action:u32,
        action_list:Vec<String>,
        show_tree:bool,
        just_final:bool,
        leaf_only:bool,
        local_reparent:bool,
        print_style:bool,
        solve:bool) -> Self {
        let mut f = Forker {
            fork_percentage,
            max_action,
            action_list,
            show_tree,
            just_final,
            leaf_only,
            local_reparent,
            print_style,
            solve,
            root_name:String::from("a"),
            process_list:Vec::new(),
            children:HashMap::new(),
            parents:HashMap::new(),
            name_length:1,
            base_names:String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            curr_names:String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            curr_index:1
        };

        f.process_list.push(f.root_name.clone());
        f.children.entry(f.root_name.clone()).or_insert(vec![]);

        f
    }

    fn grow_names(&mut self) {
        let mut new_names:String = String::new();
        for b1 in self.curr_names.chars() {
            for b2 in self.base_names.chars(){
                new_names = format!("{}{}",b1,b2);
            }
        }
        self.curr_names = new_names;
        self.curr_index = 0;
    }

    fn get_name(self){
        
    }


    fn run(&self) {
        println!("{:>32}", "Process Tree:");
    }
}

pub fn pares_op(op_vec: Vec<&str>) {
    let mut fork_op = ForkOption::new();
    let mut i = 2;
    while i < op_vec.len() {
        match op_vec[i] {
            "-h" | "--help" => {
                print!("{}", HELP);
                return;
            }
            // "-s" => {
            //     fork_op.seed = op_vec[i + 1].parse().unwrap();
            //     i = i + 2;
            // }
            // "-j" => {
            //     fork_op.jobs = op_vec[i + 1].parse().unwrap();
            //     i = i + 2;
            // }
            // "-l" => {
            //     fork_op.jlist = op_vec[i + 1].to_string();
            //     i = i + 2;
            // }
            // "-m" => {
            //     fork_op.maxlen = op_vec[i + 1].parse().unwrap();
            //     i = i + 2;
            // }
            // "-p" => {
            //     fork_op.policy = op_vec[i + 1].to_string();
            //     i = i + 2;
            // }
            // "-q" => {
            //     fork_op.quantum = op_vec[i + 1].parse().unwrap();
            //     i = i + 2;
            // }
            // "-c" => {
            //     fork_op.solve = true;
            //     i = i + 1;
            // }
            _ => {
                println!("fork_op_parse match wrong!!");
                return;
            }
        }
    }
    execute(fork_op)
}

struct ForkOption {
    // seed: u64,
// fork_percentage: f64,
// action_list: Vec<u32>,
// actions
// show_tree
// print_style
// just_final
// leaf_only
// local_reparent
// solve
}

impl ForkOption {
    fn new() -> Self {
        ForkOption {}
    }
}

fn execute(options: ForkOption) {
    // let f = Forker::new();
    // f.run();
}
