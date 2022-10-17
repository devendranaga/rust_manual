# Rust Notes from a C Programmer

## Cargo

Cargo is the Rust's build system.

**create new project:**

```bash
cargo new <project-name> # creates binary project
```

Creating a new library is similar except give --lib after the project-name.

```bash
cargo new <project-name> --lib # creates library project
```

**Create and Build the project:**

```bash
cd <project-name>
cargo build
```

The binary will be under `target/debug/`.

Rust has a linter called `clippy` just like `clang` or `gcc` with
compiler flags or a proprietary toolset to perform static code
analysis.

To run clippy,

```rust

cargo clippy

```

## Writing unit tests in rust

Unit test facilities are within the `.rs` file itself. As one example,

```rust

#[test]
fn test_a() {
    // make a function call here to test the API that is being written
}

```

Rust compiler potentially ignores any thing below `#[test]` declaration.

`#[test]` is an attribute in Rust.

One can execute the test case by running `cargo test`.

## Introduction

**Helloworld**

```rust
fn main() {
    println!("Hello world\n");
}
```

* `fn` means function.
* `main` is the starter function.
* `println!` is a macro.
* `\n` is still valid even in Rust.


```rust
fn main() {
    let x = 13;
    println!("{}", x);
}
```

The variable `x` type is automatically known to the compiler. In most of the
times, one does not have to define the variable type.

However we can still initialize variables as well. Below are the list of basic
data types from Rust.

1. signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
2. unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`.

The `{}` within `println!`, is a format specifier.

#### Variable declaration

```rust
let v = 4;
```

means a variable that holds an integer type and contains value of 4.

or in another way,

```rust
let v: u32 = 4;
```

An unsigned 32 bit integer variable v that contains value of 4.

Return in Rust is denoted by `->` parameter. So any function that returns
something will have this set. For instance:

Below program calculates the permutations.

```rust

fn permute(val: u32) -> u32 { // -> u32 meaning this function returns an unsigned 32 bit integer
    if (val == 0) || (val == 1) {
        1
    } else {
        let mut i : u32 = 1;
        let mut res : u32 = 1;
        while (i <= val) {
            res *= i;
            i = i + 1;
        }
        res
    }
}

fn main() {
    let res = permute(4);
    println!("{}", res);
}

```

Rust does not complain if we do not write `return` statement in some cases.

Writing a return value without a semi colon implicitly meaning that 
the function returns.

Rust also does not allow us to use `++` operator. So we need to explicitly
use `i = i + 1` for increment operation or `i += 1` works.

## Strings

Comparison of Strings is done with `==` operator just like in C++.

### simple calculator program

```rust

use std::env; // to read command line arguments

fn calculator(op: String, val1: f64, val2: f64) -> f64 {
    let res : f64;

    if op == "Add" {
        res = val1 + val2;
    } else if op == "Sub" {
        res = val1 - val2;
    } else if op == "Mul" {
        res = val1 * val2;
    } else if op == "Div" {
        res = val1 / val2;
    } else {
        res = -1.0;
    }

    res
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let val1 : f64 = args[2].parse().unwrap();
    let val2 : f64 = args[2].parse().unwrap();

    let ref op = &args[1];

    calculator(op.to_string(), val1, val2);
}

```

## Keyword use

1. Import or rename items from other crates or modules.
2. Usually a use keyword is used to shorten the path required to refer to a
   module item. The keyword may appear in modules, blocks and
   even functions, usually at the top.
3. Just like `using namespace std`.

```rust

let def = "a string";

def = "new string";

```

Non Mutable means constant. By default variables are immutable.

changing the variable to other name, would result in a compiler error.
Rust variables are by default mutable. Changing the value results in a
compiler error.

```rust

let mut def = "a string";

def = "new string";

```

Allows chaging the variable.


## Bitwise operators

Same bitwise operators in C does work in Rust as well.

Below example provide a overview of the bitwise operations.

### Rotate bits

```rust

fn rotate64(val: u64, pos: u32) -> u64 {
    (val << pos) | (val << (64 - pos))
}

fn rotate32(val: u32, pos: u32) -> u32 {
    (val << pos) | (val << (32 - pos))
}

```

### Bitwise operations

```rust

fn main() {
    let a:u32 = 4;
    let b:u32 = 2;

    let result = a & b;
    println!("result & {}", result);

    let result = a | b;
    println!("result | {}", result);

    let result = a ^ b;
    println!("result ^ {}", result);

    let result = a << 2;
    println!("result << {}", result);

    let result = !a;
    println!("result ! {}", result);

    let result = a >> 2;
    println!("result >> {}", result);

    let result = a == 4;
    println!("result == {}", result);
}

```

## Arrays

Arrays in rust are denoted as the following.

```rust
let array: [type; number_of_items];

```

When assigining the elements for all elements, below expression will be enough.

```rust
let array: [u8; 10] = [0; 10];

```

An example of iterating elements in array is as follows.

```rust

fn main() {
    let mut a : [u8; 10] = [0; 10];

    a[0] = 0;
    a[1] = 1;
    a[2] = 2;

    for item in a.iter() {
        let x : &u8 = item;
        println!("{}", x);
    }
}

```

## Vectors

```rust

let mut arr:Vec<u8> = Vec::new();

```

setting an element in Vector:

```rust

arr.push(10);

```

example:

```rust

fn main() {
    let mut arr:Vec<u8> = Vec::new();

    arr.push(10);
    arr.push(20);
    arr.push(255);

    for i in &arr {
        println!("{}", i);
    }
}

```

## print macro


`println!` - is a macro that prints a new line at the end
`print!` - is a macro that doesn't print a new line at the end

printing an element will need to use `"{}"` 

To print two values, we must use `"{}"` two times.

New line can still be printed with using the `\n` character.


```rust

fn main() {
    println!("---- println example ----");

    let x = 1;
    let y = 1;

    println!("x: {}, y: {}", x, y);
}

```

## enums

Enums in rust are defined in same way as C.

Enums in rust must begin with Camelcase.

```rust

enum Fruits {
    Apple,
    Orange,
}

```

Comparing enums is generally not possible straight forward.

For example, the following code is invalid in rust.

```rust

let f : Fruits = Fruits::Apple;

if f == Fruits::Apple {
    println!("fruit is Apple");
}

```

Enums need to be compared with `match` keyword.

For example, the following code returns converted string.

```rust

fn get_string(f: Fruits) -> String {
    match f {
        Fruits::Apple => "Apple".to_string(),
        Fruits::Orange => "Orange".to_string(),
    }
}

```

# std::env

Rust provides environmental details via `std::env`. This also contain command-
line arguments.

For command line args,

```rust

let args : Vec<String> = env::args().collect();

```

to collect them in vector of strings.

```rust

use std::env;

fn main() {
    for arg in env::args() {
        println!("{}", arg);
    }
}

```

Another method is indexing and using `while` loop.

```rust

use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    let len = env::args().len();
    let mut i : usize = 0;

    while i < len {
        println!("{}", args[i]);
        i += 1;
    }
```

`std::env::args()` is part of rust standard library.
This function provides a iterator of the command line arguments passed.
So a `for` can be used to iterate over it.

Also, `collect` method can be used as well to collect the arguments into
a vector of strings.

The statement `use std::env` brings the `std::env` module in scope, so that
the member functions of the `std::env` can be used from `env` directly.

when indexing with the `for`, the first value of the collected args is always
the program name.

# Standard library

## std::io

Example of `std::io::stderr` and `std::io::stdout` and `std::io::stdin`
are described in below example.

```rust

use std::io::Write;

fn main() {
    writeln!(std::io::stderr(), "error message").unwrap();
    writeln!(std::io::stdout(), "error message").unwrap();

    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    println!("{}", buf);
}

```

The macro `writeln!` is used when namespace `std::in::Write` is brought
in scope.

the handles `stdout()` and `stderr()` are used in combination with
the `writeln!` function.

The `unwrap` method is used if incase `writeln!` fails, the program
will panic.

the stdin handle is part of `std::io::stdin()`. It contains the
`read_line` method which accepts a buffer.

buf is initialized with `String::new()` method.

# File i/o in rust

Small program that opens and read a file.

```rust
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
```

### str primitive type

the `str` type is the most primitive type.

Below are some of the implementations of the `str` type.

1. `len` : returns length of the string in bytes.
2. `is_empty` : returns true if string has 0 bytes.


```rust
fn main() {

    let string = "rustlang on fc36";
    let emptystr = "";
    let len = string.len(); // get length of the string

    println!("length {}", len);

    println!("empty str: {} {}", string.is_empty(), emptystr.is_empty());
}

```

