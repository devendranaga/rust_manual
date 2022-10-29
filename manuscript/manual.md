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

Rust variable declaration follows the `golang` type of variable declarations.

```rust
let variable_name : variable_type = value;
```

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

definition of array involve the use of `[]` operator. Within it, type and length
of array follows.

```rust

let array : [type; length]

```

Arrays always need to be initialized. An initialization format follows an equals
to sign followed by `[]`, within it defines the initial value and length of
array to initialize it with.

```rust

let array : [type; length] = [initial value; length]

```

The `length` at the end must be same as the length at the initializer.


```rust

let array : [u8; 10] = [0; 10];

```

Another way to set values in array is to use indexing operator.

Array indices are of type `usize` always. Anything other than the use of `usize`
results in compiler error.

```rust

fn main() {
    let mut a1 : [u8; 12] = [0, 12];
    let mut i : usize = 0;
    let mut val : u8 = 0;

    while i < 12 {
        a[i] = val;
        val += 1;
        i += 1;
    }

    for item in a1 {
        println!("{}", item);
    }
}

```

Converting the buffer to string `std::from_utf8(&ref buf)`. Since it may fail, make a call to `unwrap` or `expect`.

```rust
use std::str;

fn main() {
	let buf = [104, 101, 108, 108, 111];
	let s = str::from_utf8(&buf).unwrap().to_string();
	
	println!("{}", s);
}
```

Passing an array to a function as an immutable reference.

```rust
fn function(array : &[u32]);
```

## Strings

Strings are defined with the `String` type or the `str` type.

Declaring a string is as follows.

```rust
let apple = "apple".to_string();
```

Another way is,

```rust
let apple = String::from("apple");
```

For comparing two strings two of many approaches can be used. For example with `==`
 and `string.eq(string_2)`.
 
```rust
fn main() {
    let apple = "apple".to_string();
    let banana = "banana".to_string();

    println!("{}", apple == banana);
    println!("{}", apple.eq(&banana));
}

```

Joining strings can be done with the `+` or with the `.join` operator.

For example,

```rust
fn main() {
	let apple = "apple".to_string();
	let banana = "banana".to_string();

	let mut conc = apple + " " + &banana;

	println!("{}", conc);

	// comment below line or above line for compilation
	// or use

	let mut cont = [apple, banana].join(" ");

	println!("{}", cont);
}
```

To convert a string to a binary data, use `as_bytes()`.

This can be useful when sending strings over a network connection for instance.

For example,

```rust
fn main() {
	let string_data = "hello".as_bytes();
	let mut i = 0;
	
	while i < string_data.len() {
		println!("{}", string_data[i]);
		i += 1;
	}
}
```

### Conversions

conversion of `string` to `integer` or other types is possible in rust.

Converting string to integer: 

```rust
let s1 = "27".to_string();
let v_int = s1.parse::<f32>().unwrap();
```

The conversion syntax it is basically, `string.parse::<type>.unwrap()`.
Or otherwise one can do directly with the type setting.

```rust
let s1 = "27".to_string();
let v_int : i32 = s1.parse().unwrap();
```

when defining the type itself to the variable, the compiler could recognize the corresponding call to the conversion function.

Implicit is more easy to remember syntactically.

Conversion functions can crash and calls `panic!` resulting in the program break. Most of the times we do not want this behavior,
main reason is that programs run indefinitely and should not crash.

Thus, handling it is part of good programming. Here's an example of how to handle such behavior.

```rust
let s1 = "dev".to_string();

match s1.parse::<i32>() {
	Ok(val) => println!("{}", val),
	Err(why) => println!("invalid number {}", why),
}
```



## Vectors

Initialize the array of bytes.

```rust

let mut arr:Vec<u8> = Vec::new();

```

setting an element in Vector:

```rust

arr.push(10);

```

passing a Vector to a function as an immutable reference.

```rust
fn function(list : &Vec<u8>);
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

To collect args skipping the program name use `env::args().skip(1).collect()`.

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
    let path = Path::new("./src/main.rs");
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    
    let res = match file.read_to_string(&mut data) {
    	Err(why) => panic!("failed to read from file {}", why),
	Ok(res) => res,
    };
    
    println!("{}", data);
}
```

Copy contents of one file to other.

```rust
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
	let path = Path::new("./src/main.rs");
	let mut file = File::open(&path).unwrap();
	let mut file_w = File::create("foo.rs").unwrap();
	
	let mut data = String::new();
	let res = match file.read_to_string(&mut data) {
		Err(why) => panic!("failed to read from file {}", why);
		Ok(res) => match file_w.write_all(data.as_bytes()) {
			Err(why) => panic!("failed to write to file {}", why);
			Ok(res) => res,
		},
	};
}
```

**File types:**

`std::os::unix::fs::FileTypeExt` provides interfaces for unix / linux based systems for file types.
`std::fs` needed as well.

Below program shows example of file types

```rust
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::env;

fn main() {
	let args : Vec<String> = env::args().collect();
	match fs::metadata(&args[1]) {
		Err(why) => panic!("failed to get file {}", why),
		Ok(metadata) => {
			let file_type = metadata.file_type();
			
			if file_type.is_dir() == true {
				println!("file_type : directory");
			} else if file_type.is_file() == true {
				println!("file_type : file");
			} else if file_type.is_block_device() == true {
				println!("file_type : block device");
			}
		},
	};
}
```

Below are the list of file type traits available in rust.

```rust
pub trait FileTypeExt {
    fn is_block_device(&self) -> bool;
    fn is_char_device(&self) -> bool;
    fn is_fifo(&self) -> bool;
    fn is_socket(&self) -> bool;
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

### Structures

Defining structures is as follows.

```rust
struct Person {
    name : String,
    age : u32,
}

fn main() {
    let name = String::from("hello");
    let age = 32;

    let p = Person { name, age };

    let p1 = Person {
        name : "hello".to_string(),
        age : 33,
    };

    println!("p name: {} age: {}", p.name, p.age);
    println!("p1 name: {} age: {}", p1.name, p1.age);
}

```

Structure variables does not end with a semi colon like in C or C++. They end
with a comma `,`. Structure does not require a semi colon after it.

Unlike C++, defining functions within structures is prohibited by rust.

Structures in rust always require initialization before using anywhere.

Structs can be put in vectors. Below code defines the `EventData` structure and
creates a vector of event list.

```rust
struct EventData {
    evt_type : u32,
    evt_len : u32,
    ethertype: u16,
}

fn main() {
    let evt_msg1 = EventData {
        evt_type : 2,
        evt_len : 100,
        ethertype : 0x0800,
    };

    let evt_msg2 = EventData {
        evt_type : 4,
        evt_len : 101,
        ethertype : 0x0801,
    };

    let mut evt_list : Vec<EventData> = Vec::new();

    evt_list.push(evt_msg1);
    evt_list.push(evt_msg2);

    for evt in evt_list {
        println!("evt_type: {} evt_len: {} ethertype: {}",
                    evt.evt_type, evt.evt_len,
                    evt.ethertype);
    }
}

```

see the vector allocation above. uninitialized vectors are not allowed as well.

structs can be nested as well.

For example,

```rust
struct A {
    a1 : u32,
}

struct B {
    a1 : a,
}

let b1 = B {
    b1 : A { a1 : 4 }
};

```

Nested initialization requires initializing the structures full below.

The general pattern is the following

```rust

let variable_name = structure-type {
    variable_of_struct : structure-type { variable : value }
}

```

if there are non structure variables within the structure, then the following will apply.

```rust

let variable_name = structure-type {
    variable_of_struct : value
}

```

## Impl

Rust does not have objects like in C++. But it provides such option with `impl`.

For example,

```rust
struct Person {
	name : String
}

impl Person {
	fn print_name(&self) {
		println!("name is {}", self.name);
	}
}
```

See that the `struct` name and the `impl` name are same.

A more detailed explanation of using `impl` is below. The program below sets the structures from within `impl`  types of the structures. And the member functions defined by their implementations are called from `main`.

```rust
struct Timestamp {
    year : u32,
    mon : u32,
    day : u32,
    hour : u32,
    min : u32,
    sec : u32,
}

impl Timestamp {
    fn clear(&mut self) {
        self.year = 0;
        self.mon = 0;
        self.day = 0;
        self.hour = 0;
        self.min = 0;
        self.sec = 0;
    }

    fn print(&self) {
        println!("year: {} mon: {} day: {} hour: {} min: {} sec: {}",
                        self.year, self.mon, self.day, self.hour,
                        self.min, self.sec);
    }
}

struct EventLogData {
    evt_type : u32,
    ts : Timestamp,
}

impl EventLogData {
    fn clear(&mut self) {
        self.evt_type = 0;
        self.ts.clear();
    }

    fn print(&self) {
        println!("evt_type: {}", self.evt_type);
        self.ts.print();
    }
}

fn main() {
    let mut evt_data = EventLogData {
        evt_type : 0,
        ts : Timestamp {
            year : 2018,
            mon : 1,
            day : 1,
            hour : 1,
            min : 1,
            sec: 1,
        },
    };

    println!("before clear\n");
    evt_data.print();
    evt_data.clear();

    println!("after clear\n");
    evt_data.print();
}

```

The `self` parameter is used to access the structure members. The `self` parameter is not really mandatory unless used. But if the `self` parameter is required, it must be always placed as first argument for the function. `self` can be mutable as well. 

For example,

```rust
struct Name {
	person : String
}

impl Name {
	fn set(&mut self, name : String) {
		self.person = name;
	}

	fn print(&self) { println!("name: {}", self.name); }
}
```

The above implementation has `set` method being passed another argument `name` of type `String`.

The calling from main is always with one argument because the type is declared. For example, the following

```rust

fn main() {
	let mut n = Name {
		person : "Dev".to_string(),
	};

	n.set("dev".to_string());
	n.print();
}
```


## RAII

RAII is Resource Acquisition Is Initialization. C++ support this by automatically releasing memory / resources when a block of code goes out of scope.

Rust does support the RAII as well. For example,

```rust
fn create_str() {
	let str = String::new();
}

fn main() {
	create_str();
}
```

In `create_str`  the variable `str` is allocated and when the function goes out of scope, its automatically freed up. We do not have to free it. 

Running a `valgrind` produces no leaks proving that the memory is automatically freed.

## Ownerships

Ownerships are very important concepts in rust.  For example,

```rust
fn main() {
	let a = 4;
	let b = a;
}
```

Is only a copy of variable a into b as they are not allocated on the heap.

now consider a being allocated on heap.

```rust
fn main() {
	let a1 = Box::new(4u32);
	let b1 = a1;
}
```

This assignment `let a1 = Box::new(4u32)` allocates a1 on the heap and sets value 4.
The assingment `b1 = a1` moves the ownership from a1 to b1.
Any accesses to a1 now results in a compiler error as the reference is moved.

And no need to free b1 because the compiler will automatically insert the free code, soon the variable goes out of scope.

Even the mutability and ownerships is changed when the ownership is transfered,

For example,

```rust
fn main() {
	let var = Box::new(4u32);

	println!("{}", var);
	let mut var1 = var;

	*var1 = 14;
	println!("{}", var1);
}
```

## Time

Duration is part of `std::time` API which is used for many purposes, for example used in `sleep`.


```rust
use std::{thread, time}

fn main() {
	let one_second = time::Duration::from_secs(1);
	let one_millisecond = time::Duration::from_millis(1000);
	let one_microsecond = time::Duration::from_micros(1000000);
	
	thread::sleep(one_second);
	thread::sleep(one_millisecond);
	thread::sleep(one_microsecond);
}
```

### Sleep

Sleep is part of rustlang standard library. `std::time` provides duration and `std::thread` provides `sleep`.

```rust
use std::{thread, time}

fn main() {
	let one_second = time::Duration::from_millis(1000);
	
	println!("start");
	thread::sleep(one_second);
	println!("stop");
}
```




## Sockets

It would be really nice to make syscalls directly from rust layer. But rust already offers them in the standard library.
Sockets and network functionality is present in `std::net`.

### Udp sockets

Udp sockets implemented in `std::net::UdpSocket`.

Below program sends one packet over the lo interface over 1235.

```rust
use std::net::UdpSocket;

fn main() {
	let socket = UdpSocket::bind("127.0.0.1:1234").unwrap();
	let send_buf : String = String::from("hello");
	
	socket.send_to(send_buf.as_bytes(), "127.0.0.1:1235").unwrap();
}
```

One can monitor with `nc` on linux with `nc -l --udp 1235` to listen for the udp payload.

The method `send_to` accepts two arguments,

1. send buffer as `u8 []`.
2. destination address in `server:port` string format.

Writing a UDP Server is similar.

```rust
use std::str;
use std::net::UdpSocket;

fn main() {
	let socket = UdpSocket::bind("127.0.0.1:1234").unwrap();
	let mut buf = [0; 100];
	
	let (amt, _src) = socket.recv_from(&mut buf).unwrap();
	let rx_data = &mut buf[..amt];
	let rx_msg = str::from_utf8(rx_data).unwrap().to_string();
	
	println!("{}", rx_msg);
}
```

One can send message with `nc` on linux with `nc 127.0.0.1 1234` and write text on the console to send to the server.

The `src` is set as `_src` instead as we are not using it. Any variable that is not used in rust must be prefixed with `_`.


## Generics

Generics are similar to macros and templates. For example one can write them the same way as template programming in C++.

For example the below program computes minimum of two numbers of any types.

```rust

fn min<T>(a : T, b : T) -> T {
	if a < b {
		a
	} else {
		b
	}
}

fn main() {
	let o = min(1, 2);
	let o1 = min(1.1, 2.2);
	println!("{} {}", o, o1);
}
```

However, the above statement is too generic, meaning that strings cannot be compared that way or any structure types unless there's some specific overloaded implementation.

Rust compiler fails to compile this due to a large set of possible types for T.

So, instead the suggest we get from the compiler is to use `std::cmp::PartialOrd` to be added after T in the function definition.

Adding it will look like below,

```rust

fn min<T: std::cmp::PartialOrd(a: T, b: T) -> T {
	if a < b {
		a
	} else {
		b
	}
}

fn main() {
	let o = min(1, 2);
	let o1 = min(1.1, 2.2);
	println!("{} {}", o, o1);
}
```



## Hardware

