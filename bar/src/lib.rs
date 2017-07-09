#![feature(conservative_impl_trait)]

extern crate futures;

use futures::{future, Future};

pub fn foo() -> impl Future<Item=String, Error=()> {
    future::ok(()).and_then(|()| future::ok(format!("Hello, {}!", "world")))
}
