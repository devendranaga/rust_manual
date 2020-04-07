fn boolean_example() {
    let logical: bool = true;

    if logical {
        println!("its true\n");
    } else {
        println!("its false\n");
    }
}

fn match_check() {
    let logical: bool = true;

    match logical {
        true => println!("its true"),
        false => println!("its false"),
    }
}

fn char_types_example() {
    let mut char_types = 'c';
    println!("{}", char_types);
}

fn char_types_deepr() {
    let char_types = 'd';

    let mut res = char_types.is_alphabetic();
    if res {
        println!("is alphabetic");
    } else {
        println!("is not alphabetic");
    }

    res = char_types.is_lowercase();
    if res {
        println!("is lowercase");
    } else {
        println!("is uppercase");
    }

    let alnum = '7';

    res = alnum.is_alphanumeric();
    if res {
        println!("is alphanumeric");
    } else {
        println!("is not alphanumeric");
    }
}

fn main() {
    boolean_example();
    match_check();
    char_types_example();
    char_types_deepr();
}
