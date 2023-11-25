// Hide warnings
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Alternatively, instead of nested match statements
    // (unwrap and expect call the same match arms as above - expect lets us adjust the error message)
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Or,
    let greeting_file = File::open("hello.txt").unwrap();

    // Or,
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // Error propagation
    let res = read_username_from_file();
    match res {
        Ok(_) => println!("{:?}", res),
        Err(e) => panic!("Oops {:?}", e),
    }

    // Shorthand error propagation
    let res = read_username_from_file_2();
    match res {
        Ok(_) => println!("{:?}", res),
        Err(e) => panic!("Oops {:?}", e),
    }

    // Chaining ? operators
    let res = read_username_from_file_3();
    match res {
        Ok(_) => println!("{:?}", res),
        Err(e) => panic!("Oops {:?}", e),
    }

    // In-built error handling
    let res = read_username_from_file_4();
    match res {
        Ok(_) => println!("{:?}", res),
        Err(e) => panic!("Oops {:?}", e),
    }

    // Option error handling
    let res = read_username_from_file_4();
    let res = match res {
        Ok(txt) => txt,
        Err(e) => panic!("Oops {:?}", e),
    };
    let my_char = last_char_of_first_line(&res);
    match my_char {
        Some(item) => println!("{:?}", item),
        None => println!("Could not find char"), 
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    // The ? operator placed after a Result value
    // will propagate (i.e., return) errors to be
    // handled in the calling function
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    // NB: The ? operator can only be used in functions whose
    // return type is compatible with the value the ? is used on.
    // In this case we return a Result<T, E> which is compatible.
    // Compatible types are: 
    // Result, Option, or another type that implements FromResidual
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}