use std::ops::Deref;
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum SharedList {
    SharedCons(i32, Rc<SharedList>),
    SharedNil,
}

use crate::List::{Cons, Nil}; // brings the Cons and Nil variants into scope from the List enum defined in the current crate
use crate::SharedList::{SharedCons, SharedNil};

struct MyBox<T>(T); // tuple struct with one element of type T

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: std::fmt::Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("Dereferencing value: {}", &self.0);
        &self.0 // .0 accesses the first vaue in a tuple struct
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // Automatically called when variable goes out of scope
        // Here is where you would place any logic that you wanted to run when an
        // instance of your type goes out of scope.
        println!("Dropping CustomSmartPointer with data {}", self.data)
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

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    // Dropping a Value Early with std::mem::drop
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // Reference counters for shared refs
    let a = Rc::new(SharedCons(5, Rc::new(SharedCons(10, Rc::new(SharedNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = SharedCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = SharedCons(4, Rc::clone(&a)); // Rc::clone() increases ref count; not deep copy
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // the implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
