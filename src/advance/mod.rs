pub mod heap;
pub mod adder;
pub mod concurrency;
pub mod error;
pub mod city;
pub mod guarantee;
pub mod ffi;
pub mod algorithm;


pub fn demo() {
    heap::demo();
    concurrency::demo();
    error::demo();
    //    journey::city::demo();
    guarantee::demo();
    algorithm::demo();
}