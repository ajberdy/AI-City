extern crate ws;

use ws::listen;

/// A WebSocket echo server
fn main() {
    listen("127.0.0.1:3012", |out| {
        move |msg| {
            out.send(msg)
        }
    });
}
