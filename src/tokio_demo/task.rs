use std::time::Duration;

use futures::prelude::*;
use futures::future::{self, Either};

pub struct Timeout;

impl Timeout {
    pub fn new<T>(_: T, _: Duration) -> Box<Future<Item=(), Error=()>> {
        unimplemented!()
    }
}

pub struct MyExecutor;

impl MyExecutor {
    pub fn spawn<T>(&self, _: T) {
        unimplemented!()
    }
}
pub struct Error;

pub fn demo() {}