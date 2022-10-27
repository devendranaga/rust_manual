use std::env;

fn largest_uint(list : &Vec<u32>) -> &u32 {
    let mut largest = &list[0];

    for val in list {
        if val > largest {
            largest = val;
        }
    }

    largest
}

fn main() {
    let args : Vec<String> = env::args().skip(1).collect();
    let mut list : Vec<u32> = Vec::new();

    for arg in args {
        let var : u32 = arg.parse().unwrap();

        list.push(var);
    }

    let largest = largest_uint(&list);
    println!("{}", largest);
}
