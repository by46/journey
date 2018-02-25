use advance::adder::add_two;
use advance::adder::*;

#[test]
//#[should_panic(expected = "assertion failed")]
fn it_works() {
    assert_eq!("Hello", "Hello");
}


#[test]
fn test_add_two() {
    assert_eq!(4, add_two(2));
}

#[test]
#[ignore]
fn expensive_test() {}