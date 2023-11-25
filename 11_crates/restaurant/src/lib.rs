// Hide warnings
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

// Nesting paths
// use std::{cmp::Ordering, io};
// use std::io::{self, Write};

// Bring all public items in to scope 
// use std::collections::*;

// Module path (either from different file or commented below)
pub use crate::front_of_house::hosting; 

mod front_of_house;

// mod front_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order(); // super <-> path to parent module
//     }

//     fn cook_order() {}

//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

mod back_of_house {
    pub struct Breakfast {
        // Only specified fields are public
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            // Since the struct has a private field, we need this public constructor
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        // All variants are public
        Soup,
        Salad,
    }
}

// pub use crate::front_of_house::hosting; // re-exporting with pub - external code can call reataurant::hosting::<fn_in_hosting> instead of the full path

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Due to use keyword
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod customer {
    use crate::front_of_house::hosting; // only applies to same scope it is defined in

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); 
    }
}