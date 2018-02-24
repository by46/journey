fn closure_demo() {
    let plus_one = |x: i32| x + 1;
    assert_eq!(2, plus_one(1));

    let plus_two = |x| {
        let mut result: i32 = x;
        result += 1;
        result += 1;
        result
    };
    assert_eq!(3, plus_two(1));

    let plus_three = |x: i32| -> i32{ x + 3 };
    assert_eq!(4, plus_three(1));

    let num = 5;
    let plus_num = |x: i32| x + num;
    assert_eq!(10, plus_num(5));

    let num2 = 5;
    let plus_num_2 = |x: i32| x + num2;
    assert_eq!(10, plus_num_2(5));
    let _y = &num2;


    let mut num3 = 5;
    {
        let mut add_num = move |x: i32| num3 += x;
        add_num(5);
    }
    assert_eq!(5, num3);
}

fn closure_demo2() {
    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(10, num);
}

fn closure_demo3() {
    fn call_with_one<F>(some_closure: F) -> i32
        where F: Fn(i32) -> i32 {
        some_closure(1)
    }
    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);
}

fn closure_demo4() {
    fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }
    let answer = call_with_one(&|x| x + 2);
    assert_eq!(3, answer)
}

fn closure_demo5() {
    fn call_with_ref<F>(some_closure: F) -> i32
        where F: for<'a> Fn(&'a i32) -> i32 {
        let value = 1;
        some_closure(&value)
    }
    let answer = call_with_ref(|x| x + 2);
    assert_eq!(3, answer)
}

fn closure_demo6() {
    fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    let answer = call_with_one(&add_one);
    assert_eq!(2, answer)
}

fn closure_demo7() {
    fn factory() -> Box<Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }

    let f = factory();
    let answer = f(1);
    assert_eq!(6, answer)
}

pub fn demo() {
    closure_demo();
    closure_demo2();
    closure_demo3();
    closure_demo4();
    closure_demo5();
    closure_demo6();
    closure_demo7();
}