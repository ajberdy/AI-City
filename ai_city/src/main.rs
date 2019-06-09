pub mod grid_world_capnp {
    include!(concat!(env!("OUT_DIR"), "/grid_world_capnp.rs"));
}


pub mod client;
pub mod server;

pub fn main() {
    println!("main.rs");

    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "client" => return client::main(),
            "server" => return server::main(),
            _ => ()
        }
    }
    println!("usage: {} ADDRESS[:PORT]", args[0]);
}
