use crate::algorithms::Create;

pub struct Square {}

impl Create for Square {
    fn create(args: crate::arguments::Arguments) -> () {
        println!("Hello! I am going to make a square...");
    }
}