#[derive(Copy, Clone)]
enum T {
    T1 = 1,
    T2,
}


struct S {
    t : T
}

fn main() {
    println!("Hello, world!");
    let array : [u8; 3 ] = [1, 12, 3];

    const s : [S; 2] = [
        {
            S {
                t : T::T1
            }
        },
        {
            S {
                t : T::T2
            }
        }
    ];

    let mut i : usize = 0;

    while i < s.len() {
        println!("t : { }", (s[i].t) as u32);
        i = i + 1;
    }
}
