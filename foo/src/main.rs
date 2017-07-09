#![feature(conservative_impl_trait)]

extern crate bar;
extern crate futures;

use futures::{future, Future};

fn foo() -> impl Future<Item=String, Error=()> {
    future::ok(()).and_then(|()| future::ok(format!("Hello, {}!", "world")))
}

fn main() {
    println!("foo: {}", foo().wait().unwrap());
    println!("bar::foo: {}", bar::foo().wait().unwrap());
}
