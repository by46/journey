use std::cell::Cell;

pub fn tuple_demo() {
    let addr = (1, "hello");
    println!("address {:?}", addr);
    println!("address[1] {}", addr.1);
    let (no, name) = addr;
    println!("no {} {}", no, name);
}


pub fn vector_demo() {
    let mut v = vec![1, 2, 5, 4];
    let v2 = vec![0; 10];
    println!("v: {:?}, v2: {:?}", v, v2);
    println!("Second element {}", v[1]);

    let i: usize = 0;
    println!("Third element {}", v[i]);
    match v.get(10) {
        Some(x) => println!("Item 10 is {}", x),
        None => println!("Sorry, this vector is too short")
    }

    // 你不能在使用 vector 的所有权遍历之后再次遍历它
    for element in &v {
        println!("A reference to {}", element);
    }
    for element in &v {
        println!("A reference to {}", element);
    }
    v.push(8);
    for element in &v {
        println!("A reference to {}", element);
    }

    let result = borrow_demo(&v, &v2);
    println!("Result borrow : {}", result);

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let answer = foo(&v1, &v2);
    println!("vector sum :{}", answer);
}

pub fn slice_demo() {
    let ages = [1, 2, 3, 4, 5, 6];
    println!("ages {:?}", ages);
    let partition = &ages[1..3];
    println!("partition ages {:?}", partition);
}


struct Person<'a> {
    x: &'a i32,
}

impl<'a> Person<'a> {
    fn x(&self) -> &'a i32 { self.x }
}


fn diverges() -> ! {
    panic!("this function never returns!");
}


// Rust 确保了对于任何给定的资源都正好（只）有一个绑定与之对应

pub fn borrow_demo(_v1: &Vec<i32>, _v2: &Vec<i32>) -> i32 {
    42
}

pub fn mutable_demo() {
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("mutable_demo {}", x);
}

pub fn sum_vec(v: &Vec<i32>) -> i32 {
    return v.iter().fold(0, |a, &b| a + b);
}

pub fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let s1 = sum_vec(v1);
    let s2 = sum_vec(v2);
    s1 + s2
}

pub fn skip_prefix<'a, 'b>(line: &'a str, _prefix: &'b str) -> &'a str {
    line
}

pub fn life_demo() {
    let line = "lang:en=Hello World!";
    let lang = "en";

    let v;
    {
        let p = format!("lang:{}=", lang);  // -+ `p` comes into scope.
        v = skip_prefix(line, p.as_str());  //  |
    }                     // -+ `p` goes out of scope.
    println!("{}", v);
}

pub fn struct_demo() {
    {
        let y = &5;
        let mut f = Person { x: y };
        let z = &7;
        f.x = z;
        println!("struct person {}", f.x());
    }
}

pub fn mutable_demo2() {
    struct Point {
        x: i32,
        y: i32,
    }
    let mut a = Point { x: 5, y: 6 };
    a.x = 10;
    a.y = 20;
    //    let b = Point { x: 5, y: 6 };
    //    b.x = 10; Error: cannot assign to immutable field `b.x`
    struct Point2 {
        y: Cell<i32>,
    }
    let b = Point2 { y: Cell::new(6) };
    b.y.set(7);
    println!("y: {:?}", b.y);
}

pub fn struct_demo2() {
    struct Point {
        x: i32,
        y: i32,
    }
    struct PointRef<'a> {
        x: &'a mut i32,
        y: &'a mut i32,
    }
    let mut point = Point { x: 0, y: 0 };
    {
        let r = PointRef { x: &mut point.x, y: &mut point.y };
        *r.x = 5;
        *r.y = 6;
    }
    assert_eq!(5, point.x);
    assert_eq!(6, point.y);

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }
    let mut point2 = Point3D { x: 0, y: 0, z: 0 };
    point2 = Point3D { y: 1, ..point2 };
    println!("new point.y {} {} {}", point2.x, point2.y, point2.z);
}

pub fn tuple_struct_demo() {
    struct Color(i32, i32, i32);
    let black = Color(0, 2, 3);
    println!("hello world {}", black.2);
    let Color(_, g, b) = black;
    println!("(_, {}, {})", g, b);

    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
    struct Electron {};
    struct Proton;
    let _x = Electron {};
    let _y = Proton;
    //    let z = Electron; // Error
}

pub fn enum_demo() {
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }
    let x: Message = Message::Move { x: 3, y: 4 };
    let y: Message = Message::Write("hello, world".to_string());
    let z: Message = Message::ChangeColor(1, 2, 3);

    let v = vec!["hello".to_string(), "world".to_string()];
    let _v1: Vec<Message> = v.into_iter().map(Message::Write).collect();

    let messages = [x, y, z];
    for message in &messages {
        match message {
            &Message::Quit => println!("quit!"),
            &Message::ChangeColor(r, g, b) => println!("change color {}, {}, {}", r, g, b),
            &Message::Move { x, y: new_name_for_y } => println!("move cursor {} {}", x, new_name_for_y),
            //            &Message::Write(&s) => println!("Write({})", s),
            _ => println!("Write()")
        }
    }
}

pub fn match_demo() {
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else"),
    }
}