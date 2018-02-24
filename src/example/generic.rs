use example::common;
use std::fmt::Debug;
use std;

struct Point<T> where T: Debug {
    x: T,
    y: T,
}

impl<T> Point<T> where T: Debug {
    fn display(&self) {
        println!("Point<x:{:?}, y:{:?}>", self.x, self.y)
    }
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y)
    }
}

fn takes_anything<T>(x: T) {}

pub fn generic_demo() {
    common::line();

    takes_anything(0);

    let x: Option<i32> = Some(6);
    let y: Option<f64> = Some(5.0f64);

    let mut int_origin = Point { x: 1, y: 2 };
    let float_origin = Point { x: 0.0, y: 0.0 };
    int_origin.display();
    float_origin.display();
    int_origin.swap();
    int_origin.display();

}