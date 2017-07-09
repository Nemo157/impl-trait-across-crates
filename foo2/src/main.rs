#![feature(conservative_impl_trait)]

extern crate bar;
extern crate futures;

use futures::{future, Future};

fn foo2() -> impl Future<Item=String, Error=()> {
    future::ok(()).and_then(|()| {
        fn msg() -> String { format!("Hello, {}!", "world") }
        future::ok(msg())
    })
}

fn main() {
    println!("foo2: {}", foo2().wait().unwrap());
    println!("bar::foo2: {}", bar::foo2().wait().unwrap());
}
