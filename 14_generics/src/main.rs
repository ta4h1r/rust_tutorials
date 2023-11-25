// Hide warnings
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
        
    largest
}

// Refactor using generic type
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    // for item in list {
    //     if item > largest {
    //         largest = item;  // needs trait to work
    //     }
    // }

    largest
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}
impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // NB: Note that we have to declare T just after impl 
    // so we can use T to specify that we’re implementing methods on the type Point<T>
    //  By declaring T as a generic type after impl, Rust can identify that the 
    // type in the angle brackets in Point is a generic type rather than a concrete 
    // type. We could have chosen a different name for this generic parameter than 
    //the generic parameter declared in the struct definition, but using the same 
    //name is conventional
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    //  implement methods only on Point<f32> instances rather than on Point<T> instances
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Structs with multiple generic types 
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    // In method defs 
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // Mixup - some generic parameters are declared
    // with impl and some are declared with the method definition
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);





    
}