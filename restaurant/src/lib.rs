use std::fmt::Result;
use std::io::Result as IoResult;


use std::{cmp::Ordering, io, self};


#[cfg(test)]
pub mod front_of_house {

    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

    fn eat() {
        front_of_house::hosting::add_to_waitlist();
        crate::front_of_house::hosting::add_to_waitlist();
    }

}

// could not find `front_of_house` in the crate root (???)
pub use crate::front_of_house::hosting;

fn deliver_order() {}

mod back_of_house {

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

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    hosting::add_to_waitlist();
}

