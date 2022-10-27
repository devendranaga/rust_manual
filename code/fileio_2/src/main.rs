use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("./src/main.rs");
    let mut file = File::open(&path).unwrap();
    let mut file_w = File::create("foo.rs").unwrap();

    let mut data = String::new();
    let res = match file.read_to_string(&mut data) {
        Err(why) => panic!("failed to read from file {}", why),
        Ok(res) => match file_w.write_all(data.as_bytes()) {
            Err(why) => panic!("failed to write {}", why),
            Ok(res) => res,
        },
    };

    println!("{}", data);
}
