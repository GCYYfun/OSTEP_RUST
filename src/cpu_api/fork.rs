use rand::{Rng, SeedableRng};
use std::collections::HashMap;

const HELP: &str = include_str!("help.txt");

fn random_randint(seed: u64, low: usize, hi: usize) -> usize {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let rand: f64 = rng.gen();
    return (low as f64 + rand * (hi - low + 1) as f64) as usize;
}

fn random_choice(seed: u64, l: &Vec<char>) -> char {
    let index = random_randint(seed, 0, l.len() - 1);
    return l[index];
}

struct Forker {
    fork_percentage: f64,
    max_action: usize,
    action_list: String,
    show_tree: bool,
    just_final: bool,
    leaf_only: bool,
    local_reparent: bool,
    print_style: String,
    solve: bool,
    root_name: char,
    process_list: Vec<char>,
    children: HashMap<char, Vec<char>>,
    parents: HashMap<char, char>,
    name_length: u32,
    base_names: Vec<char>,
    curr_names: Vec<char>,
    curr_index: usize,
}

impl Forker {
    fn new(
        fork_percentage: f64,
        max_action: usize,
        action_list: String,
        show_tree: bool,
        just_final: bool,
        leaf_only: bool,
        local_reparent: bool,
        print_style: String,
        solve: bool,
    ) -> Self {
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
            root_name: 'A',
            process_list: Vec::new(),
            children: HashMap::new(),
            parents: HashMap::new(),
            name_length: 1,
            base_names: Vec::new(),
            curr_names: Vec::new(),
            curr_index: 1,
        };

        f.process_list.push(f.root_name);
        f.children.insert(f.root_name, vec![]);

        let alphabet = (b'A'..b'z' + 1)
            .map(|c| c as char)
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<_>>();

        f.base_names = alphabet;
        f.curr_names = f.base_names.clone();
        f
    }

    fn grow_names(&mut self) {
        let mut new_names: Vec<char> = Vec::new();
        for b1 in &self.curr_names {
            for b2 in &self.base_names {
                new_names.push(format!("{}{}", b1, b2).chars().next().unwrap());
            }
        }
        self.curr_names = new_names;
        self.curr_index = 0;
    }

    fn get_name(&mut self) -> char {
        if self.curr_index == self.curr_names.len() {
            self.grow_names();
        }

        let name = self.curr_names[self.curr_index];
        self.curr_index += 1;
        name
    }
    // ,p,level,pmask,is_last
    fn walk(&mut self, p: char, level: usize, pmask: &mut HashMap<usize, bool>, is_last: bool) {
        print!("{:35}", " ");

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
            for i in 0..level - 1 {
                if *pmask.get(&i).unwrap() {
                    print!("{:<3}", chars.0);
                } else {
                    print!("{:3}", " ");
                }
            }
            if *pmask.get(&(level - 1)).unwrap() {
                if is_last {
                    print!("{}{}{} ", chars.3, chars.1, chars.1);
                } else {
                    print!("{}{}{} ", chars.2, chars.1, chars.1);
                }
            } else {
                print!("{}{}{} ", chars.1, chars.1, chars.1);
            }
        }
        println!("{}", p);

        if is_last {
            &pmask.insert(level - 1, false);
        }

        &pmask.insert(level, true);

        if let Some(c) = self.children.get(&p).unwrap().last() {
            let last: char = c.clone();
            for child in self.children.get(&p).unwrap().clone() {
                if child == last {
                    self.walk(child, level + 1, pmask, true);
                    break;
                }
                self.walk(child, level + 1, pmask, false);
            }
        }
    }

    fn print_tree(&mut self) {
        self.walk(self.root_name.clone(), 0, &mut HashMap::new(), false);
    }

    fn do_fork(&mut self, p: char, c: char) -> String {
        self.process_list.push(c);
        self.children.insert(c, Vec::new());
        self.children.get_mut(&p).unwrap().push(c);
        self.parents.insert(c, p);
        return format!("{} forks {}", p, c);
    }

    fn collect_children(&mut self, p: char) -> Vec<char> {
        if self.children.get(&p).unwrap().is_empty() {
            return vec![p];
        } else {
            let mut l = vec![p];
            for c in self.children[&p].clone() {
                let mut vc = self.collect_children(c);
                l.append(&mut vc);
            }
            return l;
        }
    }

    fn do_exit(&mut self, p: char) -> String {
        let exit_parent = self.parents[&p];
        let pos = self.process_list.iter().position(|x| *x == p).unwrap();
        self.process_list.remove(pos);

        if self.local_reparent {
            for orphan in self.children.get(&p).unwrap().clone() {
                self.parents.insert(orphan, exit_parent);
                self.children.get_mut(&exit_parent).unwrap().push(orphan);
            }
        } else {
            let mut descendents = self.collect_children(p);
            let pos = descendents.iter().position(|x| *x == p).unwrap();
            descendents.remove(pos);
            for d in descendents {
                self.children.insert(d, vec![]);
                self.parents.insert(d, self.root_name);
                self.children.get_mut(&self.root_name).unwrap().push(d);
            }
        }

        let pos = self
            .children
            .get(&exit_parent)
            .unwrap()
            .iter()
            .position(|x| *x == p)
            .unwrap();
        self.children.get_mut(&exit_parent).unwrap().remove(pos);
        // self.children[p] = -1 # should never be used again
        // self.parents[p] = -1  # should never be used again

        return format!("{} EXITS", p);
    }

    fn bad_action(&self, action: &str) {
        panic!(
            "bad action {}, must be X+Y or X- where X and Y are processes",
            action
        )
    }

    fn check_legal(&self, action: &str) -> Vec<char> {
        if action.contains(&"+") {
            let tmp: Vec<&str> = action.split("+").collect();
            if tmp.len() != 2 {
                self.bad_action(action);
            }
            return vec![
                tmp[0].chars().next().unwrap(),
                tmp[1].chars().next().unwrap(),
            ];
        } else if action.contains(&"-") {
            let tmp: Vec<&str> = action.split("-").collect();
            if tmp.len() != 2 {
                self.bad_action(action);
            }
            return vec![tmp[0].chars().next().unwrap()];
        } else {
            self.bad_action(action);
            return vec![];
        }
    }

    fn run(&mut self, seed: u64) {
        println!("{:>32}", "Process Tree:");
        self.print_tree();
        println!();

        // let mut action_list: Vec<&str> = Vec::new();
        let mut action_list: Vec<String> = Vec::new();
        let als: String;
        let mut actions = 0;
        if !self.action_list.is_empty() {
            als = self.action_list.clone();
            action_list = als
                .split(",")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
        } else {
            let mut temp_process_list = vec![self.root_name];
            let mut level_list: HashMap<char, usize> = HashMap::new();
            level_list.insert(self.root_name, 1);
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
            while actions < self.max_action {
                // let rand: f64 = rand::thread_rng().gen();

                let rand: f64 = rng.gen();
                if rand < self.fork_percentage {
                    let fork_choice = random_choice(seed, &temp_process_list);
                    let new_child = self.get_name();
                    // let s:String = format!("{}+{}", fork_choice, new_child);
                    action_list.push(format!("{}+{}", fork_choice, new_child));
                    temp_process_list.push(new_child)
                } else {
                    let exit_choice = random_choice(seed, &temp_process_list);
                    if exit_choice == self.root_name {
                        continue;
                    }
                    let pos = temp_process_list
                        .iter()
                        .position(|&x| x == exit_choice)
                        .unwrap();
                    temp_process_list.remove(pos);
                    action_list.push(format!("{}-", exit_choice));
                }
                actions = actions + 1;
            }
        }

        for a in action_list.clone() {
            let mut _action: String = String::new();
            let tmp = self.check_legal(&a);
            if tmp.len() == 2 {
                let fork_choice = tmp[0];
                let new_child = tmp[1];
                if !self.process_list.contains(&fork_choice) {
                    self.bad_action(&a);
                }
                _action = self.do_fork(fork_choice, new_child);
            } else {
                let exit_choice = tmp[0];
                if !self.process_list.contains(&exit_choice) {
                    self.bad_action(&a);
                }

                if self.leaf_only && self.children.get(&exit_choice).unwrap().len() > 0 {
                    _action = format!("{} EXITS (failed: has children)", exit_choice);
                } else {
                    _action = self.do_exit(exit_choice);
                }
            }

            if self.show_tree {
                if self.solve {
                    println!("Action: {}", _action);
                } else {
                    println!("Action?")
                }

                if !self.just_final {
                    self.print_tree();
                }
            } else {
                println!("Action:{}", _action);
                if !self.just_final {
                    if self.solve {
                        self.print_tree();
                    } else {
                        println!("Process Tree?");
                    }
                }
            }
        }

        if self.just_final {
            if self.show_tree {
                println!();
                println!("{:>32}", "Final Process Tree:");
                self.print_tree();
                println!();
            } else {
                if self.solve {
                    println!();
                    println!("{:>32}", "Final Process Tree:");
                    self.print_tree();
                    println!();
                } else {
                    println!();
                    println!("{:>32}", "Final Process Tree?");
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
            "-s" => {
                fork_op.seed = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-f" => {
                fork_op.fork_percentage = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-A" => {
                fork_op.action_list = op_vec[i + 1].to_string();
                i = i + 2;
            }
            "-a" => {
                fork_op.actions = op_vec[i + 1].parse().unwrap();
                i = i + 2;
            }
            "-t" => {
                fork_op.show_tree = true;
                i = i + 1;
            }
            "-F" => {
                fork_op.just_final = true;
                i = i + 1;
            }
            "-L" => {
                fork_op.leaf_only = true;
                i = i + 1;
            }
            "-R" => {
                fork_op.local_reparent = true;
                i = i + 1;
            }
            "-c" => {
                fork_op.solve = true;
                i = i + 1;
            }
            _ => {
                println!("fork_op_parse match wrong!!");
                return;
            }
        }
    }
    execute(fork_op)
}

struct ForkOption {
    seed: u64,
    fork_percentage: f64,
    action_list: String,
    actions: usize,
    show_tree: bool,
    print_style: String,
    just_final: bool,
    leaf_only: bool,
    local_reparent: bool,
    solve: bool,
}

impl ForkOption {
    fn new() -> Self {
        ForkOption {
            seed: 0,
            fork_percentage: 0.7,
            action_list: String::from(""),
            actions: 5,
            show_tree: false,
            print_style: String::from("fancy"),
            just_final: false,
            leaf_only: false,
            local_reparent: false,
            solve: false,
        }
    }
}

fn execute(options: ForkOption) {
    println!("");
    println!("ARG seed : {}", options.seed);
    println!("ARG fork_percentage : {}", options.fork_percentage);
    println!("ARG actions : {}", options.actions);
    println!("ARG action_list : {}", options.action_list);
    println!("ARG show_tree : {}", options.show_tree);
    println!("ARG just_final : {}", options.just_final);
    println!("ARG leaf_only : {}", options.leaf_only);
    println!("ARG local_reparent : {}", options.local_reparent);
    println!("ARG print_style : {}", options.print_style);
    println!("ARG solve : {}", options.solve);
    println!("");

    let mut f = Forker::new(
        options.fork_percentage,
        options.actions,
        options.action_list,
        options.show_tree,
        options.just_final,
        options.leaf_only,
        options.local_reparent,
        options.print_style,
        options.solve,
    );
    f.run(options.seed);
}
