use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};

use crate::grid_world_capnp::grid_world;

use futures::Future;
use tokio::io::AsyncRead;


pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} client HOST:PORT", args[0]);
        return;
    }

    try_main(args).expect("ugh");
}

fn try_main(args: Vec<String>) -> Result<(), ::capnp::Error> {
    use std::net::ToSocketAddrs;

    let mut runtime = ::tokio::runtime::current_thread::Runtime::new().unwrap();

    let addr = args[2].to_socket_addrs()?.next().expect("could not parse address");
    let stream = runtime.block_on(::tokio::net::TcpStream::connect(&addr)).unwrap();
    stream.set_nodelay(true)?;
    let (reader, writer) = stream.split();

    let network =
        Box::new(twoparty::VatNetwork::new(reader, std::io::BufWriter::new(writer),
                                           rpc_twoparty_capnp::Side::Client,
                                           Default::default()));
    let mut rpc_system = RpcSystem::new(network, None);
    let grid_world: grid_world::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
    runtime.spawn(rpc_system.map_err(|_e| ()));

    let request = grid_world.ping_request();

    let promise = request.send();
    let response = runtime.block_on(promise.promise)?;

    let pong = response.get()?.get_pong()?;
    println!("{}", pong);
    Ok(())
}
