use std::cell::Cell;
use std::cell::RefCell;

use example::common;

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
}