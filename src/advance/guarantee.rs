use std::cell::Cell;
use std::cell::RefCell;
use std::fmt::Display;
use std::borrow::Borrow;

use example::common;

fn foo<T: Borrow<i32> + Display>(a: T) {
    println!("a is borrowed: {}", a)
}

fn foo2<T: AsRef<str> + Display>(a: T) {
    println!("foo2 {}", a)
}

pub fn demo() {
    common::line();

    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("x {}", x.get());

    let i = RefCell::new(vec![1, 2, 3]);
    {
        println!("{:?}", i.borrow());
    }
    {
        let mut mut_ref = i.borrow_mut();
        mut_ref.push(5);
    }
    println!("{:?} ", i);

    let mut i = 5;
    foo(&i);
    foo(&mut i);

    let s = "hello".to_string();
    foo2(s);
}