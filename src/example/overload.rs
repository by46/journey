use std::ops::Add;
use std::ops::Mul;
use std::ops::Deref;
use std::fmt::Write;

use example::common;

trait HasArea<T> {
    fn area(&self) -> T;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point2 {
    x: i32,
    y: i32,
}

struct Square<T> {
    x: T,
    y: T,
    side: T,
}

struct DereferenceExample<T> {
    value: T,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Add<i32> for Point2 {
    type Output = f64;
    fn add(self, rhs: i32) -> f64 {
        1.0
    }
}

impl<T> HasArea<T> for Square<T>
    where T: Mul<Output=T> + Copy {
    fn area(&self) -> T {
        self.side * self.side
    }
}

impl<T> Deref for DereferenceExample<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

fn get_log_state() -> i32 { 3 }

macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
    }
    };
}
macro_rules! foo {
    (x=> $e:expr) => (println!("mod X: {}", $e));
    (y=> $e:expr) => (println!("mod Y: {}", $e));
}

macro_rules! o_O {
    (
        $(
            $x:expr; [ $( $y:expr ),* ]
        );*
    ) => {
        &[ $( $( $x+$y),* ),*]
    };
}

macro_rules! log {
    ($msg:expr) => {{
        let state: i32 = get_log_state();
        if state > 0 {
            println!("log({}): {}", state, $msg);
        }

    }};
}

macro_rules! write_html {
    ($w:expr,) => (());
    ($w:expr, $e:tt) => (write!($w, "{}", $e));
    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)* ) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}

pub fn demo() {
    common::line();

    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("{:?}", p3);

    let p1 = Point2 { x: 1, y: 0 };
    println!("{:?}", p1 + 64);

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 12.0f64,
    };
    println!("Area of s: {}", s.area());

    let x = DereferenceExample { value: 'a' };
    assert_eq!('a', *x);

    assert_eq!(vec2![1,2,3], [1, 2, 3]);
    foo!(y=>4);
    foo!(y=>4+4);
    let a: &[i32] = o_O!(10; [1,2,3];20;[4,5,6]);
    assert_eq!(a, [11, 12, 13, 24, 25, 26]);

    let state = "hello world";
    log!(state);

    let mut out = String::new();
    write_html!(&mut out,
    html[
        head[title["macros guide"]]
        body[h1["macros are the best!"]]
    ]
    script[div["hello"]]);
    println!("{}", out);

}