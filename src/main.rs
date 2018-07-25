#![feature(rust_2018_preview)]
#[warn(unused_imports)]

use std::error::Error;

extern crate sentry;
use sentry::integrations::failure::capture_error;
use sentry::integrations::panic::register_panic_handler;
use sentry::protocol::*;

use sentry::backtrace_support::current_stacktrace;

fn test() {
    sentry::capture_event(Event {
        message: Some("Hello test!".into()),
        level: Level::Info,
        stacktrace: current_stacktrace(),
        ..Default::default()
    });
}
fn main() {
    let _guard = sentry::init("https://key@sentry.io/1248884");
    sentry::capture_message("Hello World!", sentry::Level::Info);
    register_panic_handler();
    test();
    // code using sentry goes here.
    sentry::capture_event(Event {
        message: Some(format!("{} {}\n{}", file!(), line!(), format!("{:?}", Backtrace::new())).into()),
        level: Level::Info,
        ..Default::default()
    });
    panic!("hello world");
}
