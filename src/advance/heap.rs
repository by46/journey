use example::common;

fn foo(x: &i32) {
    let y = 10;
    let z = &y;

    baz(z);
    bar(x, z);
}

fn bar(_a: &i32, _b: &i32) {
    let _c = 5;
    let d = Box::new(5);
    let e = &d;

    baz(e);
}

fn baz(f: &i32) {
    let _g = 100;
}

pub fn demo() {
    common::line();
    let h = 3;
    let i = Box::new(20);
    let j = &h;

    foo(j);
}