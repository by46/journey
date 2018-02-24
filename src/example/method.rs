use std;
use example::common;

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
    // 关联函数就是静态方法
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, radius, y }
    }
    fn display(&self) {
        println!("Circle<x:{}, y:{}, radius: {}, area: {}>", self.x, self.y, self.radius, self.area())
    }
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 1.0 }
    }
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }
    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }
    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }
    fn finalize(&self) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius }
    }
}

pub fn method_demo() {
    common::line();
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    c.display();

    let d = c.grow(2.0);
    d.display();

    let e = Circle::new(0.0, 5.0, 2.0);
    e.display();

    let f = CircleBuilder::new()
        .x(1.0)
        .y(2.0)
        .radius(2.0)
        .finalize();
    f.display();
}