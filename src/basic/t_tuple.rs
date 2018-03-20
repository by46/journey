fn tuple_basic() {
    let tuple = (1, 2, "name");
    println!("First {}", tuple.0);
    println!("Last {}", tuple.2);
}

pub fn demo() {
    tuple_basic();
}