use std::collections::VecDeque;

use std::io;
use futures::Future;
use futures::Async;
use futures::prelude::*;
use time;
use tokio;
use tokio_core::reactor::Core;

#[derive(Debug)]
pub struct Widget;

fn poll_widget() -> Async<Widget> {
    let now = time::now();
    println!("current date {:?}", now);
    if now.tm_min < 25 {
        Async::NotReady
    } else {
        Async::Ready(Widget {})
    }
}

pub struct MyTask;

impl Future for MyTask {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<()>, ()> {
        match poll_widget() {
            Async::Ready(widget) => {
                println!("widget={:?}", widget);
                Ok(Async::Ready(()))
            }
            Async::NotReady => {
                Ok(Async::NotReady)
            }
        }
    }
}

pub struct SpinExecutor {
    tasks: VecDeque<Box<Future<Item=(), Error=()>>>,
}

impl SpinExecutor {
    pub fn spawn<T>(&mut self, task: T)
        where T: Future<Item=(), Error=()> + 'static {
        self.tasks.push_back(Box::new(task));
    }
    pub fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop_front() {
            match task.poll().unwrap() {
                Async::Ready(_) => {}
                Async::NotReady => {
                    self.tasks.push_back(task)
                }
            }
        }
    }
}

pub struct Doubler<T> {
    inner: T,
}

pub fn double<T>(inner: T) -> Doubler<T> {
    Doubler { inner }
}

impl<T> Future for Doubler<T>
    where T: Future<Item=usize> {
    type Item = usize;
    type Error = T::Error;

    fn poll(&mut self) -> Result<Async<usize>, T::Error> {
        //        match self.inner.pool()? {
        //            Async::Ready(v) => Ok(Async::Ready(v * 2)),
        //            Async::NotReady => Ok(Async::NotReady),
        //        }
        let v = try_ready!(self.inner.poll());
        Ok(Async::Ready(v * 2))
    }
}

pub struct Adder;

impl Future for Adder {
    type Item = u8;
    type Error = ();

    fn poll(&mut self) -> Result<Async<u8>, ()> {
        let now = time::now();
        println!("current date {:?}", now);
        if now.tm_min < 50 {
            Ok(Async::NotReady)
        } else {
            Ok(Async::Ready(7))
        }
    }
}

pub fn demo() {
    let adder = Adder {};
    let mut core = Core::new().unwrap();
    core.run(adder);
}