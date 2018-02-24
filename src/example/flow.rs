pub fn if_demo() {
    let x = 6;
    let y = if x == 4 { x + 1 } else { x + 5 };
    println!("if condition {}", y);
}

pub fn for_demo() {
    for (index, value) in (5..10).enumerate() {
        println!("index={}, value={}", index, value);
        if value == 7 {
            break;
        }
    }
    let lines = "hello\nworld".lines();
    for (index, line) in lines.enumerate() {
        if index == 0 {
            continue;
        }
        println!("line {}: {}", index, line);
    }

    'outer: for i in 0..10 {
        'inner: for j in 0..10 {
            if i % 2 == 0 {
                continue 'outer;
            }
            if j % 2 == 0 {
                continue 'inner;
            }
            println!("x: {}, y: {}", i, j);
        }
    }
}