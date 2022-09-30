# Rust from a C Programmer

## Cargo

Cargo is the Rust's build system.

**create new project:**

```bash
cargo new <project-name>
```

**Create and Build the project:**

```bash
cd <project-name>
cargo build
```

The binary will be under `target/debug/`.

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

## print macro


`println!` - is a macro that prints a new line at the end
`print!` - is a macro that doesn't print a new line at the end

printing an element will need to use `"{}"` 

to print two values, we must use `"{}"` two times


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

