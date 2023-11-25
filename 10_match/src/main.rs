// Hide warnings for unused code.
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)] // so we can inspect the state in a minute
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // Matching custom Enums
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    // Matching Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(&six, &none);

    // Catch-all: The last pattern will match all values not specifically listed
    let dice_roll = 255; // type inferrred from match arms
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // Discard catch-all value
    let dice_roll = -9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // Do nothing catch-all
    let dice_roll = -9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // Consider only one pattern to match, ignore catch-all
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Or equivalently for conciseness
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // Consider one pattern to match, handle catch-all
    let mut count = 0;
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Or equivalently for conciseness
    let mut count = 0;
    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn add_fancy_hat() {
    println!("Add fancy hat")
}
fn remove_fancy_hat() {
    println!("Remove fancy hat")
}
fn move_player(num_spaces: u8) {
    println!("Moved player {num_spaces} spaces")
}
fn reroll() {
    println!("Reroll")
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
