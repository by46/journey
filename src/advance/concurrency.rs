use std::thread;
use std::time::Duration;
//use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

use example::common;

fn demo1() {
    thread::spawn(|| {
        println!("hello from thread");
    });
    thread::sleep(Duration::from_millis(50));
}

fn demo2() {
    let handler = thread::spawn(|| { "hello from thread 2" });
    println!("{}", handler.join().unwrap());
}

fn demo3() {
    let x = 1;
    let handler = thread::spawn(move || {
        println!("x is {}", x)
    });
    handler.join().unwrap();
}

fn demo4() {
    let mut data = Arc::new(Mutex::new(vec![1, 2, 3]));
    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[0] += i
        });
    }
    thread::sleep(Duration::from_millis(50));
    let result = data.clone();
    let container = result.lock().unwrap();
    println!("demo4 {}", container[0])
}

fn demo5() {
    let mut data = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(i).unwrap();
        });
    }
    for _ in 0..10 {
        print!("{},", rx.recv().unwrap());
    }
    println!();
    let data = data.clone();
    let container = data.lock().unwrap();
    println!("demo5 result {}", container);
}

fn demo6() {
    let handler = thread::spawn(|| {
        panic!("boom")
    });
    let result = handler.join();
    println!("demo6 result {}", result.is_err());
}

pub fn demo() {
    common::line();
    demo1();
    demo2();
    demo3();
    demo4();
    demo5();
    demo6();
}