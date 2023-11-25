#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]   
// #[cfg(test)] specifies that this module should not be 
// compiled unless we run `cargo test`, and not when we run
// `cargo build``
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn another () {
        panic!("Make this test fail")
    }
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore]
    fn it_adds_two() {
        // When the assertions fail, these macros print their 
        // arguments using debug formatting, which means the values 
        // being compared must implement the PartialEq and Debug traits
        assert_eq!(4, add_two(2));
        assert_ne!(1, add_two(2));
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        // Printing a custom error message 
        // (just add your message as extra arguments)
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    // We make sure that the function panics for the right reason, 
    // by adding what we 'expect' to see in the panic message.
    // i.e., "less than or equal to 100" is a substring of the 
    // panic message we expect to see.
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        // We may also use the result type instead of assertions
        // Writing tests so they return a Result<T, E> enables you 
        // to use the question mark operator in the body of tests, 
        // which can be a convenient way to write tests that should 
        // fail if any operation within them returns an Err variant.
        // You can’t use the #[should_panic] annotation on tests 
        // that use Result<T, E>
        // To assert that an operation returns an Err variant, 
        // don’t use the question mark operator on the Result<T, E>
        // value. Instead, use assert!(value.is_err()).
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn internal() {
        // We may test private functions directly if we wish to
        // since we are allowed to access all ancestor methods 
        // from within a child module  
        assert_eq!(4, internal_adder(2, 2));
    }

}
