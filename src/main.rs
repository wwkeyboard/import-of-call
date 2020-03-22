// We have to specify the crate name as if it was a different create, even though it's in our lib.rs
//use import_of_call::restaurant;
use import_of_call::*;
// Gives us access to the contents of tiki_bar.rs
mod tiki_bar;

fn main() {
    println!("Welcome to Import of Call");

    // This and the next call are equivilent
    import_of_call::dock::supplies::restock();
    // Here we can use `dock` because of the `use import_of_call::dock` statement above
    dock::supplies::restock();

    // This is callable because we import everything within the library create using `use import_of_call::*;`
    eat_at_restaurant();
    // We could also call the same function in this way, without having made the import first(but we probably wouldn't do it that way :).
    import_of_call::eat_at_restaurant();

    // Now we move on to calling functions that live in a module that's within a directory in this crate.
    // We can call this because above we `use import_of_call::*;` for everything in the library crate, and within lib.rs we call `pub mod restaurant` which brings in everything from /restaurant/mod.rs.
    restaurant::lobby();

    // We have to specify the module here since that's all we imported with the `mod tiki_bar` statement above.
    tiki_bar::drinks::serve();
}
