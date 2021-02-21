use rand::{Rng, SeedableRng};
use std::collections::HashMap;

const HELP: &str = include_str!("help.txt");

struct Forker {
    fork_percentage:f64,
    max_action:u32,
    action_list:String,
    show_tree:bool,
    just_final:bool,
    leaf_only:bool,
    local_reparent:bool,
    print_style:String,
    solve:bool,
    root_name:char,
    process_list:Vec<char>,
    children:HashMap<char,Vec<char>>,
    parents:HashMap<char,char>,
    name_length:u32,
    base_names:Vec<char>,
    curr_names:Vec<char>,
    curr_index:usize,
}

impl Forker {
    fn new(fork_percentage:f64,
        max_action:u32,
        action_list:String,
        show_tree:bool,
        just_final:bool,
        leaf_only:bool,
        local_reparent:bool,
        print_style:String,
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
            root_name:'a',
            process_list:Vec::new(),
            children:HashMap::new(),
            parents:HashMap::new(),
            name_length:1,
            base_names:Vec::new(),
            curr_names:Vec::new(),
            curr_index:1
        };
        
        f.process_list.push(f.root_name);
        f.children.insert(f.root_name,vec![]);

        let alphabet = (b'A' .. b'z' + 1)
            .map(|c| c as char)            
            .filter(|c| c.is_alphabetic()) 
            .collect::<Vec<_>>();          

        f.base_names = alphabet;
        f
    }

    fn grow_names(&mut self) {
        let mut new_names:Vec<char> = Vec::new();
        for b1 in &self.curr_names {
            for b2 in &self.base_names {
                new_names.push(format!("{}{}",b1,b2).chars().next().unwrap());
            }
        }
        self.curr_names = new_names;
        self.curr_index = 0;
    }

    fn get_name(&mut self) -> char{
        if self.curr_index == self.curr_names.len() {
            self.grow_names();
        }

        let name = self.curr_names[self.curr_index];
        self.curr_index += 1;
        name
    }   
    // ,p,level,pmask,is_last
    fn walk(&mut self,p:char,level:usize,pmask:&mut HashMap<usize, bool>,is_last:bool) {
        print!("{:35}"," ");

        let chars = ("│", "─", "├", "└");
        // if self.print_style == "basic" {
        //     for i in 0..level {
        //         print!("{:3}"," ");
        //     }
        //     println!("{:>3}",p);
        //     for child in self.children[p] {
        //         self.walk(child, level + 1, {}, False)
        //     }
        // }

        if level > 0 {
            for i in 0..level-1 {
                if *pmask.get(&i).unwrap() {
                    print!("{:<3}",chars.0);
                }else 
                {
                    print!("{:3}"," ");
                }
            }
            if *pmask.get(&(level-1)).unwrap() {
                if is_last {
                    print!("{}{}{} ",chars.3,chars.1,chars.1);
                }
                else 
                {
                    print!("{}{}{} ",chars.2,chars.1,chars.1);
                }
            }else {
                print!("{}{}{} ",chars.1,chars.1,chars.1);
            }
        }
        print!("{}",p);

        if is_last {
            &pmask.insert(level-1,false);
        }

        &pmask.insert(level, true);

        let c = &self.children.get(&p).unwrap().pop().unwrap();
        for child in *self.children.get(&p).unwrap() {
            self.walk(child, level + 1, pmask, false);
        }
        self.walk(*c, level + 1, pmask, true);
        self.children.get(&p).unwrap().push(*c);
    }

    fn print_tree(&mut self) {
        self.walk(self.root_name.clone(), 0, &mut HashMap::new(), false);
    }

    fn do_fork(&mut self,p:char,c:char) -> String{
        self.process_list.push(c);
        self.children.insert(c, Vec::new());
        self.children.get(&p).unwrap().push(c);
        self.parents.insert(c, p);
        return format!("{} forks {}",p,c);
    }

    fn collect_children(&self,p:char) {
        if self.children.get(&p).unwrap().is_empty() {
            return 
        }else {

        }
    }

    fn bad_action(&self,action:&str) {
        panic!("bad action {}, must be X+Y or X- where X and Y are processes",action)
    }

    fn check_legal(&self,action:&str) -> Vec<char>{
        if action.contains(&"+") {
            let tmp:Vec<&str> = action.split("+").collect();
            if tmp.len() != 2 {
                self.bad_action(action);
            }
            return vec![tmp[0].chars().next().unwrap(), tmp[1].chars().next().unwrap()];

        }else if action.contains(&"-"){
            let tmp:Vec<&str> = action.split("-").collect();
            if tmp.len() != 2 {
                self.bad_action(action);
            }
            return vec![tmp[0].chars().next().unwrap()];
        }
        else {
            self.bad_action(action);
            return vec![];
        }
    }


    fn run(&mut self,seed:u64) {
        println!("{:>32}", "Process Tree:");
        self.print_tree();
        println!();

        let mut action_list:Vec<&str> = Vec::new();
        let actions = 0f64;
        if !self.action_list.is_empty() {
            action_list = self.action_list.split(",").collect();
        }else {
            let _temp_process_list = vec![self.root_name];
            let mut level_list :HashMap<char,usize> = HashMap::new();
            level_list.insert(self.root_name, 1);

            while actions < self.fork_percentage {
                let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
                let rand:f32 = rng.gen();
                // if rand < self.fork_percentage {

                // }
            }
        }
        
        for a in action_list {
            let mut action:String = String::new();
            let tmp = self.check_legal(a);
            if tmp.len() == 2 {
                let fork_choice = tmp[0];
                let new_child = tmp[1];
                if !self.process_list.contains(&fork_choice) {
                    self.bad_action(a);
                }
                action = self.do_fork(fork_choice, new_child);
            }else {
                let exit_choice = tmp[0];
                if !self.process_list.contains(&exit_choice) {
                    self.bad_action(a);
                }

                if self.leaf_only && self.children.get(&exit_choice).unwrap().len() > 0 {
                    action = format!("{} EXITS (failed: has children)",exit_choice);
                }else{
                    // action = format!();
                }
            }

            if self.show_tree {
                if self.solve {
                    println!("Action: {}",action.clone());
                }else {
                    println!("Action?")
                }

                if !self.just_final {
                    self.print_tree();
                }
            }else {
                println!("Action:{}",action);
                if !self.just_final {
                    if self.solve {
                        self.print_tree();
                    }else {
                        println!("Process Tree?");
                    }
                }
            }
        }

        if self.just_final {
            if self.show_tree {
                println!();
                println!("{:>32}","Final Process Tree:");
                self.print_tree();
                println!();
            }else{
                if self.solve {
                    println!();
                    println!("{:>32}","Final Process Tree:");
                    self.print_tree();
                    println!();
                }else{
                    println!();
                    println!("{:>32}","Final Process Tree?");
                    println!();
                }
            }
        }
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
