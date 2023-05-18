# Basics

* Rust deals with low-level details of memory management, data representation and concurrency
* designed to guide one naturally towards reliable code that is efficient in terms of speed and memory usage

#### tools
* `rustc` --> compiler. takes Rust code and turns it into machine code
* `rustup` --> a command line utility that allows one to install and update Rust
* `cargo` --> Rust build system and package manager

#### build a new project using cargo
* navigate to the directory that will be the paprent of the project directory
* run the following

~~~bash
cargo new projectName
~~~

* navigate into `projectName`; you will notice the `src` directory. this is similar to _src_ in a JS project
* `Cargo.toml` is the Rust equivalent of `package.json` file in a JS project
* `main.rs` is the default entry point/file for the app

#### run your application
* navigate to the `projectName` directory
* use one of two ways
    * `cargo run` --> compiles code and runs binary 

        ~~~bash
        cargo run
        ~~~

    * `rustc` --> compiles code; you must run the binary

        ~~~bash
        rustc ./path/to/entry/file
        ./main
        ~~~

#### about
* `args` function and `Args` struct are the Rust equivalent of `argc` and `argv` in C. they are in the `env` module of Rust's `std` library
    * function returns an instance of the struct
    * import them into the workspace viz

    ~~~rust
    use std::env::{args, Args};
    ~~~

* rust is statically typed; you must explicitly type variables. if the variable is declared using `let`, the type is inferred; no need to be explicit 
* declare variables this way
    * `let` keyword

        ~~~rust
        let x = args();
        ~~~

    * `let` keyword and explicit type

        ~~~rust
        let x: Args = args();
        ~~~

    * `let`  and `mut` keywords for _mutable_ variables

        ~~~rust
        let mut x = args.nth(0);
        ~~~

    * `const` keyword and type

    ~~~rust
    const NAME: u8 = "F Njakai";
    ~~~

    * `static` keyword and type

    ~~~rust
    static CONTINENT: u8 = "Africa";
    ~~~

    * variables, by default, are immutable; use the `mut` keyword to change that

* casing  conventions

|Object|Casing|
|:---|:---|
|variables|snake_case|
|functions|snake_case|
|files|snake_case|
|constants|SCREAMING_SNAKE_CASE|
|statics|SCREAMING_SNAKE_CASE|
|types|PascalCase|
|traits|PascalCase|
|enums|PascalCase|

* format strings this way
    * `{:?}` --> standard string formatting when using the `println!` macro (macros have a bang `!` at the end)

        ~~~rust
        println!("{:?}", x);
        ~~~

    * `{:#?}` --> pretty print formatting using `println!`

        ~~~rust
        println!("{:#?}", x);
        ~~~

* the `nth` method on the `Args` struct
    * returns the nth element of an iterator, wrapped in an _Option_
    * the zeroth element, as with `argv`, is the name of he command with which the programme is invoked; the first element is the first command-line argument
    * takes in two args: a mutable reference to `self` (like the _self_ in py) and `n`, the index of the iterator in question
    * example:

        ~~~rust
        let x args:Args = args()

        let first = args.nth(0);
        println!("{:?}", first);
        ~~~

    * option `Some()` is returned when the index in question is accessible; option `None()` is returned otherwise. the binary _panics_ when `None()` is returned
    * use the `unwrap` method to access the data wrapped in the _Option_

        ~~~rust
        let mut x args:Args = args();

        let first = args.nth(0).unwrap();
        println!("{:?}", first);
        ~~~

    * **Interesting case**

        ~~~rust
        let x mut args:Args = args()

        let first = args.nth(1).unwrap();
        let op = args.nth(2).unwrap();
        let second = args.nth(3).unwrap();
        println!("{:?} {} {}", first, op, second);
        ~~~

        * say you pass 1 + 2 to the code above viz

        ~~~text
        cargo run -- 1 + 2
        ~~~

        you realise that the code panics. it panics because of the following behaviour
        > code picks up `1` on the first pass. the _"argv array"_ (`Some()` Option) now has two elements: "+" and "2"
        > code realises element 2 in `args.nth(2).unwrap()` does not exist; ~~panik~~
        > code panics
        * if you pass 1 + 2 3 4 5 6 7 8 to the code the output is

        ~~~text
        "1" 3 7
        ~~~

        because the code picks `1` on the first pass, `3` on the second and `7` on the third
        > `Some` looks like this on the first pass: ["main", 1", "+","2","3","4","5","6","7", "8"] --> `1` is at `nth(1)`
        > on the second pass: ["+", "2", "3", "4", "5", "6", "7", "8"] --> `3` is at `nth(2)`
        > on the third pass: ["4", "5", "6", "7", "8"] --> `7` is at `nth(3)`
        > recall: variable `args` it is _mutable_
        * the solution, therefore, is to use index zero always viz

        ~~~rust
        let mut args: Args= args();

        let first = args.nth(1).unwrap();
        let op = args.nth(0).unwrap();
        let second = args.nth(0).unwrap();

        println!("{#?}, {}, {}", first, op, second);
        ~~~

* _panic!_ --> is how Rust _"errors out"_ (throws errors) during runtime
* types `char` and `str` are different
    * `str` (aka _string slice_)is a string literal. stored on the stack. unmutable (its value cannot change and size is fixed at compile time)
    * use the _address-of_ operator to access a part/slice of an array of ``

        ~~~rust
        let my_string = String::from("The quick brown fox jumps over the lazy dog");
        let my_str: &str = &my_string[4..9]; //"quick"

        let my_arr: [usize; 5] = [1, 2, 3, 4, 5];
        let my_arr_slice: &[usize] = &my_arr[0..3]; //[1,2,3]
        ~~~

    * `char` is a USV (Unicode Scalar Value). it is represented in unicode with values like U+221E – the unicode for '∞'. think of a collection or array of `char`s as a string

        ~~~rust
        let my_str: &str = "Hello, world!";
        let collection_of_chars: &str = my_str.chars().as_str()
        ~~~

    * `String` is a struct. stored on heap. mutable (can have unknown size at compile time)
* number types in Rust
    * unsigned ints: `u8`,`u16`,`u32`,`u64`,`u128` (read: N-bit unsigned). positive whole numbers
    * signed ints: `i8`,`i16`,`i32`,`i64`,`i128` (read: N-bit signed). positive and negative whole numbers
    * floating point: `f32`, `f64` (read: N-bit float). positive and negative fractions (numbers with decimal points eg 1.618)
* structs --> custom data type.  used to group related data
    * example: struct `String`

    ~~~rust
    struct String{
        vec: Vec<u8>,
    }
    ~~~

    struct `String` has a field `vec` which is a `Vec` of `u8`s; `Vec` is a dynamically-sized array
    * declare an instance of a struct by giving values to the fields

    ~~~rust
    struct MyStruct{
        field_1: u8,
    }

    let my_struct = MyStruct{field_1: 1618};
    ~~~

    * struct `String` uses the function `from` to create a `String` from `&str`. this is possible because the `from` function is implemented inside `String`

    ~~~rust
    impl String{
        fn from(s: &str) -> Self{
            String{
                vec: Vec::from(s.as_bytes()),
            }
        }
    }
    ~~~

    the `Self` keyword is used in place of the type of the struct
    * other ways to write structs

    ~~~rust
    struct MyUnitStruct;
    struct MyTupeStruct(u8, u8);
    ~~~

* enums in Rust
    * useful for acting as types and as values
    ~~~rust
    enum MyErrors{
        BrainTooTired,
        TimeOfDay(String),
        CoffeeCupEmpty,
    }

    fn work() -> Result<(), MyErrors>{//Result is an enum also
        match state{
            "missing semi-colon" => Err(MyErrors::BrainTooTired),
            "15:00" => Err(MyErrors::TimeOfDay("too early to work".to_string())),
            "12:30" => Err(MyErrors::TimeOfDay("too late to work".to_string())),
            "empty" => Err(MyErrors::CoffeeCupEmpty),
            _ => Ok(())
        }
    }
    ~~~

* macros in Rust --> code which writes other code
    * similar to a function
        * must be called; in this case, with a bang `!` ~~(see what i did there?)~~
        * take in arguments
    * not similar to a function
        * takes in a variable number of arguments
    
    ~~~rust
    let my_str = "Hello, world!";
    println!("{}", my_str);

    let i_am_error = true;
    if(i_am_error){
        panic!("There was an error");
    }
    ~~~

* ownership in Rust
    * is how Rust gets away with not having a typical garbage collector whilst also not requiring the programmer to explicitly manage memory
    * three rules of ownership
        1. each value in Rust has a variable called its _owner_
        2. there can only be one owner at a time ~~highlander rules~~
        3. the value is dropped when the owner goes out of scope
    
    ~~~rust
    fn main() { // first_string is not declared yet -> has no value
    let first_string = String::from("F Njakai"); // first_string owns the value "F Njakai"
    let second_string = first_string; // second_string now owns the value "F Njakai"

    println!("Hello, {}!", first_string); // first_string is not valid because the value was moved to second_string
    }
    ~~~

    to fix this, assign `second_string` viz

    ~~~rust
    let second_string: &String = &first_string; // first_string is still the owner of the value "F Njakai"
    ~~~

    the ampersand (address-of operator), `&`, indicates that the value is a reference; that is, `second_string` no longer takes ownership of "F Njakai" but, instead, points to/contains the same address in memory as `first_string`
