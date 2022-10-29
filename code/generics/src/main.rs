
fn min<T: std::cmp::PartialOrd>(a : T, b : T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    let o = min(1, 2);
    let o1 = min(2.1, 4.0);
    println!("{} {}", o, o1);
}
