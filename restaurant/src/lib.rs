mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// abs relative reference
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

// pub struct enum
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// use
pub fn eat_at_restaurant3() {
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();
}

// pub use
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant4() {
    hosting::add_to_waitlist();
}

mod middle_of_house;

pub use crate::middle_of_house::hosting as mid_posting;

pub fn eat_at_restaurant5() {
    mid_posting::add_to_waitlist();
}

pub use crate::middle_of_house::hosting2;

pub fn eat_at_restaurant6() {
    hosting2::add_to_waitlist();
}