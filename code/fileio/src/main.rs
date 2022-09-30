// For file i/o
use std::fs::File;
// dunno what this is though
use std::io::prelude::*;
// File path
use std::path::Path;

fn main() {
    // create a path to the desired file
    let path = Path::new("./main.rs");
    let display = path.display(); /* display class / object may be? */

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file {} : {}", display, why),
        Ok(file) => file,
    }; // why semicolon needed here?

    // doesn't seem to allocate memory here though does it?
    let mut s = String::new();
    // reads entire file into a string
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read file {} : {}", display, why),
        // whats _ ?
        Ok(_) => print!("{} contains {}", display, s),
    } // why semicolon not needed here?
}
