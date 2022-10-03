
fn main() {

    let mut a: [u8; 10] = [0; 10];

    a[0] = 1;
    a[1] = 2;
    a[2] = 1;

    for item in a.iter() {
        let x: &u8 = item;
        println!("{}", x);
    }
}
