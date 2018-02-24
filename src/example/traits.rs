use std;
use std::io::Write;
use example::common;

trait HasArea {
    fn area(&self) -> f64;
    fn is_larger(&self, &Self) -> bool;
}

trait Display {
    fn display(&self);
}

trait ApproxEqual {
    fn approx_equal(&self, other: &Self) -> bool;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn is_larger(&self, other: &Self) -> bool {
        self.area() > other.area()
    }
}

impl Display for Circle {
    fn display(&self) {
        println!("Circle<x:{}, y:{}, radius: {}, area: {}>", self.x, self.y, self.radius, self.area())
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn is_larger(&self, other: &Self) -> bool {
        self.area() > other.area()
    }
}

impl Display for Square {
    fn display(&self) {
        println!("Square<x:{}, y:{}, side: {}, area: {}>", self.x, self.y, self.side, self.area())
    }
}


impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

impl ApproxEqual for f32 {
    fn approx_equal(&self, other: &Self) -> bool {
        (self - other).abs() <= std::f32::EPSILON
    }
}

fn print_area<T: HasArea + Display>(shape: T) {
    shape.display();
    println!("This shape has an area of {}", shape.area())
}

pub fn demo() {
    common::line();
    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };
    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64
    };
    print_area(c);
    print_area(s);

    let mut r = Rectangle {
        x: 0,
        y: 0,
        width: 47,
        height: 47,
    };
    assert!(r.is_square());

    r.height = 42;
    assert!(!r.is_square());
    println!("{}", 1.0.approx_equal(&1.0000001));

    let mut f = std::fs::File::create("foo.txt").ok().expect("Couldn’t create foo.txt");
    let buf = b"whatever";
    let result = f.write(buf);
}