// use std::future::Future;
// use std::task::{Context, Poll};
// use std::{future, pin::Pin};

fn main() {}

// fn main() {
//     // let waker = Waker::from(); // You would typically create a Waker here
//     let mut ctx = Context::from_waker(futures::task::noop_waker_ref());
//     let v = poll(&mut ctx);
//     println!("v: {:?}", v);
// }

// fn poll(ctx: &mut Context<'_>) -> Poll<usize> {
//     let mut fut = future::ready(2);
//     let p = Pin::new(&mut fut);
//     my_ready!(p.poll(ctx))
// }

// #[macro_export]
// macro_rules! my_ready {
//     ($expr:expr) => {
//         match $expr {
//             std::task::Poll::Ready(v) => std::task::Poll::Ready(v),
//             std::task::Poll::Pending => return std::task::Poll::Pending,
//         }
//     };
// }
