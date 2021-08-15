use crate::arguments::Arguments;

pub mod square;

pub trait Create {
    fn create(args: Arguments) -> ();
}