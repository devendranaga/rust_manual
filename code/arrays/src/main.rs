
fn main() {

    let mut a: [u8; 10] = [0; 10];
    let mut a1 : [u8; 12] = [0; 12];
    let mut i : usize = 0;
    let mut val : u8 = 0;

    a[0] = 1;
    a[1] = 2;
    a[2] = 1;

    for item in a.iter() {
        let x: &u8 = item;
        println!("{}", x);
    }

    while i < 12 {
        a1[i] = val;
        val += 1;
        i += 1;
    }

    for item in a1 {
        println!("{}", item);
    }
}
