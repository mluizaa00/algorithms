#![allow(dead_code)]

mod util;
mod sequence;

fn main() {
    //fibonacci::sequence(20);
    // fibonacci_clock::clock(Some(vec!(3, 4)),None,Some(vec!(3)),);

    sequence::kaprekar::kaprekar(321);
}
