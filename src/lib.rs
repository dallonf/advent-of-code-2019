#[macro_use]
extern crate lazy_static;
extern crate rayon;

mod prelude;

pub mod days;
mod logic;
mod util;

pub use logic::intcode;
