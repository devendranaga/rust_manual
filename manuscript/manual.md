# Rust

## Introduction

**Helloworld**

```rust
fn main() {
    println!("Hello world\n");
}
```

`fn` means function.
`main` is the starter function.
`println!` is a macro.
`\n` is still valid even in Rust.

## cargo

**create new project:**

```bash
cargo new <project-name>
```

```bash
cd <project-name>
cargo build
```

## types

1. signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
2. unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

```rust

let def = "a string";

def = "new string";

```

Non Mutable means constant. By default variables are immutable.

changing the variable to other name, would result in a compiler error. Rust variables are by default mutable. Changing the value results in a compiler error.

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


## conditional

```rust
if condition {
.. statements ..
} else {
.. statements ..
}
```

**bool type**:

`bool` is represented the same way like  C++. It has two values `true` and `false`.

**character type**:

1. is_lowercase:

```rust

let mut char_types = 'c';

let res = char_types.is_lowercase();
if res {
    println!("is lowercase");
} else {
    println!("is uppercase");
}

```

2. is_alphabet:

```rust
let mut char_types = 'c';

let res = char_types.is_alphabet();
if res {
    println!("is alphabet");
} else {
    println!("is not an alphabet");
}

```

3. is_alphanumeric:

```rust
let alnum = '7';

let res = alnum.is_alphanumeric();
if res {
    println!("is alphanumeric");
} else {
    println!("is not alphanumeric");
}

```

## std library

## std::string::String

Creating a string can be done more than one way

```rust

let str = "Hello";

```
`std::string::String` expose the following methods.

1. **new** - create new string

```rust

let mut s = String::new();

```

2. **push_str** - push string into the string

```rust

let mut s = String::new();
s.push_str("Hello");

```

second version of push is,

```rust

let mut s = String::new();
let s1 = "Hello";

s.push_str(s1);

let s3 = "rust";

s.push_str(s3);

```

3. **reserve** - reserve string size

```rust

let mut s = String::new();

s.reserve(40);

```

4. **capacity** - return string capacity

```rust

let mut s = String::new();

s.reserve(40);

println!("capacity {}", s.capacity());

```

5. **is_empty** - validate if string is empty

```rust

let mut s = String::new();

s.reserve(40);

if s.is_empty() == true {
    println!("empty string");
} else {
    println!("non empty string");
}

```

6. **len** - get len of the string


```rust

let mut s = String::new();
println!("len {}", s.len());

```

**Example**:

```rust

use std;

fn string_details(s : &String)
{
    println!("capacity {}", s.capacity());
    println!("empty {}", s.is_empty());
    if s.is_empty() == false {
        println!("\tstring {}", s);
    }
    println!("length {}", s.len());
}

fn main() {
    // create new empty string
    let mut s = String::new();
    string_details(&s);

    s.reserve(40);
    string_details(&s);

    // create a string on stack
    let s1 = "Hello";
    s.push_str(s1);
    string_details(&s);

    s.push_str(" rust programming");
    string_details(&s);

    let bytes = s.into_bytes();

    print!(" bytes ");
    for i in &bytes {
        print!("{} ", i);
    }
    println!("\n");
}

```



### std::process

**std::process::id**:

```rust
use std::process

println!("the pid : {}", process::id());

```

**struct std::process::Command**:

This structure is used to represent and maange child proc. A child proc is created via this structure. The calling convention is similar to the builder pattern.

members exposed by `Command`:

| member function | description |
|-----------------|-------------|
| `new` | create new process |
| `arg` | argument |
| `spawn` | spanw the process |
| `expect` | expect the child process failure or success |


```rust
use std::process;

let child = process::Command::new("/bin/ls")
                    .spawn()
                    .expect("failed to execute ls");

let output = child
            .wait_with_output()
            .expect("failed to wait on child process");

let res = output.status.success();
println!("{}", res);

```

the arguments are passed via the `arg`.

```rust

use std::process;

let child = process::Command::new("/bin/ls")
                    .arg("-l")
                    .spawn()
                    .expect("failed to execute ls");

let output = child
            .wait()
            .expect("failed to wait on child process");

let res = output.status.success();
println!("{}", res);

```

always `wait` must be called in a long running process so that process that's created is cleanedup.

### std::net

**std::net::TcpStream**:

| `connect` | connect to a ip and port |

```rust

use std::net::TcpStream;

fn main() {
    let mut conn = TcpStream::connect("127.0.0.1:4444");

    match conn {
        Ok(conn) => {
            println!("connected");
        }
        Err(e) => {
            println!("connection failed");
        }
    }
}

```


**std::net::TcpListener**::

|------|-------------------------|
| `bind` | bind to the ip and port |
| `accept` | accept connections |
| `incoming` | accepts incoming connections |


```rust

use std::net::TcpListener;

fn main() {
    let serv = TcpListener::bind("127.0.0.1:4444").unwrap();

    for stream in serv.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client");
            }
            Err(e) => {
                println!("failed to take client");
            }
        }
    }
}

```






