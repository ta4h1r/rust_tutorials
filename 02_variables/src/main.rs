fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Cannot infer const types, must be annotated, cannot be set to values calculated at runtime except for some trivial operations
    println!("Const {}", THREE_HOURS_IN_SECONDS);

    // Shadowing - By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Shadowing to change type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces {}", spaces);

    // Integer division truncates
    let t = -5 / 3; // Results in -1
    println!("t {}", t);

    // Tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1); // Type inference
    let (x, y, z) = tup; // Destructuring
    println!("The value of x, y, z is: {x}, {y}, {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let z = x.0;
    let o = x.1;
    let t = x.2;
    println!("The value of at pos 0, 1, 2 is: {z}, {o}, {t}");

    // Array
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Cannot change size
    let first = a[0];
    let second = a[1];
    println!("first {}, second {}", first, second);

    let a = [3; 5]; // Init the same value multiple times
    println!("a {:?}", a);
}
