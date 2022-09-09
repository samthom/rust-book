mod front_of_house; 

// this exposes crate to the user of the lib too
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitinglist();

    // Relative path
    front_of_house::hosting::add_to_waitinglist();
     
    // bringing paths into scope and using it
    hosting::add_to_waitinglist();
}

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_home() {
        hosting::add_to_waitinglist();
    }
}
