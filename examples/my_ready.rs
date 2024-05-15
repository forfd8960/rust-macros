use std::future::Future;
use std::task::{Context, Poll};
use std::{future, pin::Pin};

use macros::my_ready;

fn main() {
    // let waker = Waker::from(); // You would typically create a Waker here
    let mut ctx = Context::from_waker(futures::task::noop_waker_ref());
    let v = poll(&mut ctx);
    println!("v: {:?}", v);
}

fn poll(ctx: &mut Context<'_>) -> Poll<usize> {
    let mut fut = future::ready(2);
    let p = Pin::new(&mut fut);
    my_ready!(p.poll(ctx))
}
