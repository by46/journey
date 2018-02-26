use std::collections::HashMap;

use example::common;


pub fn demo() {
    common::line();
    let mut map = HashMap::new();
    map.insert("foo".to_string(), 42);

    assert_eq!(map.get("foo"), Some(&42));
}