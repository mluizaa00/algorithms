#![allow(dead_code)]

mod util;
mod kaprekar;
mod fibonacci;
mod fibonacci_clock;

fn main() {
    //fibonacci::sequence(20);
    // fibonacci_clock::clock(Some(vec!(3, 4)),None,Some(vec!(3)),);

    kaprekar::kaprekar();
}
