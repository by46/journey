use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;

use example::common;

#[derive(Debug)]
struct Person {
    blood_alcohol: f64,
}

#[derive(Debug)]
struct Foo {
    a: u32,
    b: &'static str,
}

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a
    }
}

impl Eq for Foo {}

impl Hash for Foo {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.a.hash(h)
    }
}

impl PartialOrd for Foo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.a.partial_cmp(&other.a)
    }
}

impl Ord for Foo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.a.cmp(&other.a)
    }
}

fn count_chars() {
    let mut count = BTreeMap::new();
    let message = "she sells sea shells by the sea shore";
    for c in message.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    assert_eq!(count.get(&'s'), Some(&8));

    println!("Number of occurrences of each character.");
    for (char, count) in &count {
        println!("{}: {}", char, count);
    }
}

fn blood_alcohol() {
    let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1];
    let mut blood_alcohol = BTreeMap::new();
    for id in orders {
        let person = blood_alcohol.entry(id).or_insert(Person { blood_alcohol: 0.0 });
        person.blood_alcohol *= 0.9;
        if person.blood_alcohol > 0.3 {
            println!("Sorry {}, I have to cut off you", id);
        } else {
            person.blood_alcohol += 0.1;
        }
    }
    for (id, person) in &blood_alcohol {
        println!("id {} : {:?}", id, person);
    }
}

fn complex_key() {
    let mut map = BTreeMap::new();
    map.insert(Foo { a: 2, b: "baz" }, 99);
    map.insert(Foo { a: 2, b: "xyz" }, 100);
    map.insert(Foo { a: 10, b: "xyz1" }, 101);

    for (key, value) in &map {
        println!("{:?}: {}", key, value);
    }
}

pub fn demo() {
    common::line();
    count_chars();
    blood_alcohol();
    complex_key();
}