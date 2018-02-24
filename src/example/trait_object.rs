use example::common;

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("String: {}", *self)
    }
}

fn do_something(f: &Foo) {
    f.method();
}

pub fn demo() {
    common::line();
    let x = 6u8;
    do_something(&x as &Foo);
    let name = "benjamin".to_string();
    do_something(&name);
}