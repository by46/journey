use std::collections::VecDeque;

use futures::Future;
use futures::Async;
use time;

pub struct Widget;

fn poll_widget() -> Async<Widget> {
    let now = time::now();
    println!("current date {}", now);
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

pub fn demo() {}