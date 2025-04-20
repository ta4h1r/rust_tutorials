use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T); // tuple struct with one element of type T

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // .0 accesses the first vaue in a tuple struct
    }
}

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x); // Pointer to the value of x, stored on the heap

    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference y, i.e., get the value out of the pointer

    // Now using our custom box instead of std Box
    // Without the Deref trait, the compiler can only dereference & references
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Deref coercion - deref() called automatically, as many times as is necessary, to match function signature
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
