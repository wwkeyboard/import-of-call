#![allow(dead_code)]

pub mod restaurant;

pub mod dock {
    pub mod supplies {
        pub fn restock() {
            food()
        }
        fn food() {
            println!("here's the food!")
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    //restaurant::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //    crate::restaurant::serving::take_order();
}
