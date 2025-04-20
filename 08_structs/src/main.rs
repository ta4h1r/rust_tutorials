// An attribute to hide warnings for unused code.
#![allow(unused_variables)]
#![allow(dead_code)]

// Custom structs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        // Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.
        self.width * self.height
    }
    fn can_hold(&self, r: &Rectangle) -> bool {
        self.area() > r.area()
    }
    fn square(size: u32) -> Self {
        // Associated function - does not work on self
        // Can alias the impl type as Self
        Self {
            width: size,
            height: size,
        }
    }
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like struct
struct AlwaysEqual;

fn main() {
    let mut user1 = build_user(String::from("foobar@baz.com"), String::from("foobar"));
    println!(
        "{}, {}, {}, {}",
        user1.email, user1.active, user1.username, user1.sign_in_count
    );
    user1.email = String::from("anotheremail@example.com");
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!(
        "{}, {}, {}, {}",
        user2.email, user2.active, user2.username, user2.sign_in_count
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 39,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("rect1 is {:#?}", rect1); // To do this we have to #[derive(Debug)] on the struct

    // dbg! takes ownership of an expression and
    // returns ownership of the value.
    // (Acts on expressions as if !dbg was not there)
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1); // Use a ref because we don't want !dbg to take
                  // ownership, otherwise rect1 will become invalid

    // Method implemenation
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // More methods
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Assicated functions
    let sq = Rectangle::square(10);
    dbg!(&sq);
    dbg!(sq.area());
    println!("{:?}", sq)
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
