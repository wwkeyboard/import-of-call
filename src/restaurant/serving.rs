use crate::restaurant::grill::*;

pub fn take_order() {
    println!("thank you for your order");
}

pub fn desert() {
    // `use crate::restaurant::grill::*;` brought this into scope
    cook::pears();
}
