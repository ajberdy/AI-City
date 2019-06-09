use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use capnp::capability::Promise;

use crate::grid_world_capnp::grid_world;

use futures::Future;
use tokio::io::AsyncRead;


fn ping(host: String) {
    use std::net::ToSocketAddrs;

    let mut runtime = ::tokio::runtime::current_thread::Runtime::new().unwrap();

    let addr = host.to_socket_addrs()?.next().expect("could not parse address");
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

    let mut request = grid_world.ping_request();
    let result = request
        .send()
        .get_response();
    println!("{}", result);
}

pub fn main() {
    ping("localhost:8000".to_string());
}
