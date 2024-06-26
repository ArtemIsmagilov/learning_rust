// use crate::front_of_house::hosting;
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    mod customer {
        pub fn eat_at_restaurant() {
            super::hosting::add_to_waitlist();
        }
    }

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use std::fmt;
// use std::io;
// fn function1() -> fmt::Result {
// }
//
// fn function2() -> io::Result<()> {
// }

// use std::fmt::Result;
// use std::io::Result as IoResult;
// fn function3() -> Result {
// }
//
// fn function4() -> IoResult<()> {
// }

use std::{
    cmp::Ordering,
    any,
};
use std::io::{
    self,
    Write
};

use std::collections::*;
