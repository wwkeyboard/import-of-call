// everything in this file is in import-of-call::restaurant.

mod grill;
pub mod serving;

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding you to the waitlist!");
        }

        fn seat_at_table() {}
    }
}

pub fn lobby() {
    sit_in_lobby();
}

fn sit_in_lobby() {
    println!("welcome to the lobby");
}
