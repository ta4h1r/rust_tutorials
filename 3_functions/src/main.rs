fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3; // Statements do not return a value
        x + 1 // Expressions (which evaluate to a resultant value) do not have a trailing semicolon
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    // Declare type after the arrow
    5 // Last expression returned implicitly
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}
