use example::common;

mod basic;
mod reading;

pub fn demo() {
    common::line();
    basic::demo();
    reading::demo();
}