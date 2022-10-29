use std::env;

fn main() {
    let args : Vec<String> = env::args().skip(1).collect();

    let var : u32 = args[0].parse().unwrap();
}

