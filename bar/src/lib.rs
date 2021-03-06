#![feature(conservative_impl_trait)]

extern crate futures;

use futures::{future, Future};

pub fn foo() -> impl Future<Item=String, Error=()> {
    future::ok(()).and_then(|()| future::ok(format!("Hello, {}!", "world")))
}

pub fn foo2() -> impl Future<Item=String, Error=()> {
    future::ok(()).and_then(|()| {
        fn msg() -> String { format!("Hello, {}!", "world") }
        future::ok(msg())
    })
}
