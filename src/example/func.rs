use example::common;

trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) {
        println!("Foo.f")
    }
}

impl Bar for Baz {
    fn f(&self) {
        println!("Bar.f")
    }
}

trait Walk {
    fn walk(&self);
}

struct Person;

impl Person {
    fn walk(&self) {
        println!("Person.walk")
    }
}

impl Walk for Person {
    fn walk(&self) {
        println!("Walk.walk")
    }
}

pub fn demo() {
    common::line();
    let baz = Baz;
//    baz.f();
    Foo::f(&baz);
    Bar::f(&baz);
    <Baz as Foo>::f(&baz);

    let person = Person;
    Person::walk(&person);
    <Person as Walk>::walk(&person);
}