use std::error::Error;

extern crate sentry;
use sentry::integrations::failure::capture_error;
use sentry::integrations::panic::register_panic_handler;
use sentry::protocol::*;
use sentry::utils::current_stacktrace;

fn test() {
    sentry::capture_event(Event {
        message: Some("Hello test!2".into()),
        level: Level::Info,
        stacktrace: current_stacktrace(),
        ..Default::default()
    });
}
fn main() {
    let _guard = sentry::init("https://d04981d09005480bb62db4ee3d02138e@sentry.io/1248884");
    sentry::capture_message("Hello World!", sentry::Level::Info);
    register_panic_handler();
    test();
    // code using sentry goes here.
    panic!("hello world");
}
