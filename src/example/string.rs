use example::common;

fn table_slice(slice: &str) {
    println!("Got: {}", slice)
}

pub fn string_demo() {
    common::line();
    let mut s = "hello".to_string();
    println!("{}", s);
    s.push_str(", world");
    println!("{}", s);

    table_slice(&s);

    let hachiko = "忠犬ハチ公";
    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }
    println!();
    for c in hachiko.chars() {
        print!("{}, ", c);
    }
    println!();

    println!("{:?}", hachiko.chars().nth(1));

    // 是字节偏移，不是字符偏移
    let dog = "hachiko";
    println!("{}", &dog[0..5]);
}