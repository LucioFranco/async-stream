#![feature(async_await)]

use async_stream::stream;

fn main() {
    stream! {
        fn foo() {
            yield "hello";
        }
    };
}
