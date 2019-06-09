use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use capnp;

use tokio::io::AsyncRead;
use tokio::runtime::current_thread;

use futures::{Future, Stream};

use crate::grid_world_capnp::grid_world;

// pub mod grid_world_capnp {
//     include!(concat!(env!("OUT_DIR"), "/grid_world_capnp.rs"));
// }



struct GridWorldImpl;

impl grid_world::Server for GridWorldImpl {
    fn ping(
        &mut self,
        _params: grid_world::PingParams,
        mut results: grid_world::PingResults
    ) -> capnp::capability::Promise<(), capnp::Error> {
        println!("pinged");
        results.get().set_pong("pong");
        capnp::capability::Promise::ok(())
    }

}

pub fn main() {
    println!("server.rs");
    use std::net::ToSocketAddrs;
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} server ADDRESS[:PORT]", args[0]);
        return;
    }

    let addr = args[2].to_socket_addrs().unwrap().next().expect("could not parse address");
    let socket = ::tokio::net::TcpListener::bind(&addr).unwrap();

    let grid_world =
        grid_world::ToClient::new(GridWorldImpl).into_client::<::capnp_rpc::Server>();

    let done = socket.incoming().for_each(move |socket| {
        socket.set_nodelay(true)?;
        let (reader, writer) = socket.split();

        let network =
            twoparty::VatNetwork::new(reader, std::io::BufWriter::new(writer),
                                      rpc_twoparty_capnp::Side::Server, Default::default());

        let rpc_system = RpcSystem::new(Box::new(network), Some(grid_world.clone().client));
        current_thread::spawn(rpc_system.map_err(|e| println!("error: {:?}", e)));
        Ok(())
    });

    current_thread::block_on_all(done).unwrap();
}
