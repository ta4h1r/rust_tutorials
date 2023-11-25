// Hide warnings for unused code.
#![allow(unused_variables)]
#![allow(dead_code)]

enum IpAddr {
    V4(u8, u8, u8, u8), // We may use any types in here, including structs and enums
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{:?}", self)
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::ChangeColor(128, 0, 255);
    m.call();

    // Option enum
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}
