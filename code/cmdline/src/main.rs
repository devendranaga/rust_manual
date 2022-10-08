use std::env;

fn main() {
    for arg in env::args() {
        println!("{}", arg);
    }

    let args : Vec<String> = env::args().collect();
    let len = env::args().len();
    let mut i : usize = 0;

    while i < len {
        println!("{}", args[i]);
        i += 1;
    }
}
