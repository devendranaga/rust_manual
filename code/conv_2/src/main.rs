use std::env;

fn main() {
    let args : Vec<String> = env::args().skip(1).collect();

    match args[0].to_string().parse::<i32>() {
        Ok(val) => println!("{}", val),
        Err(why) => println!("Value is not a string: {}", why),
    }
}
