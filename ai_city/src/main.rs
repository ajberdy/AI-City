#[macro_use]
extern crate display_derive;

pub mod city_server_capnp {
    include!(concat!(env!("OUT_DIR"), "/city_server_capnp.rs"));
}


pub mod city_client;
pub mod city_server;

pub fn main() {
    println!("main.rs");

    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "client" => return city_client::main(),
            "server" => return city_server::main(),
            _ => ()
        }
    }
    println!("usage: {} ADDRESS[:PORT]", args[0]);
}
