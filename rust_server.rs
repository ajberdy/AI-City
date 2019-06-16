// extern crate ws;

use ws::listen;

/// A WebSocket echo server
fn main() {
    listen("0.0.0.0:3012", |out| {
        move |msg| {
            out.send(msg)
        }
    })
}
