const HELP: &str = include_str!("help.txt");

pub fn help() {
    println!("{}",HELP);
}