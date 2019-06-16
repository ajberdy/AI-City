use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp, pry};
use capnp;

use tokio::io::AsyncRead;
use tokio::runtime::current_thread;

use futures::{Future, Stream};

use std::collections::HashMap;

use crate::city_server_capnp::city_server;

struct CityServerImpl {
    sessions: HashMap<String, Session>
}

impl CityServerImpl {
    pub fn new() -> CityServerImpl {
        CityServerImpl {
            sessions: HashMap::new()
        }
    }
}

impl city_server::Server for CityServerImpl {
    fn ping(
        &mut self,
        _params: city_server::PingParams,
        mut results: city_server::PingResults
    ) -> capnp::capability::Promise<(), capnp::Error> {
        println!("pinged");
        results.get().set_pong("pong");
        capnp::capability::Promise::ok(())
    }

    fn new_session(
        &mut self,
        params: city_server::NewSessionParams,
        mut results: city_server::NewSessionResults,
    ) -> capnp::capability::Promise<(), capnp::Error> {

        let uid = pry!(pry!(params.get()).get_uid());
        let session_id = format!("session-{}", uid);

        let mut session = pry!(results.get().get_session());
        session.set_session_id(&session_id.clone());
        let scene = pry!(session.get_scene());
        let mut echo = scene.init_echo();

        match self.sessions.get(&session_id) {
            Some(session) => {
                println!("found {}", session.session_id);
                echo.set_state(&session.scene.state);
            },
            None => {
                let scene = Scene{
                    state: "Start state".to_string()
                };
                echo.set_state(&scene.state);
                self.sessions.insert(session_id.clone(),
                                     Session{session_id, scene});
                println!("did not find")
            }
        }
      capnp::capability::Promise::ok(())
    }

}


struct Session {
    session_id: String,
    scene: Scene
}

struct Scene{
    state: String
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

    let city_server =
        city_server::ToClient::new(CityServerImpl::new()).into_client::<::capnp_rpc::Server>();

    let done = socket.incoming().for_each(move |socket| {
        socket.set_nodelay(true)?;
        let (reader, writer) = socket.split();

        let network =
            twoparty::VatNetwork::new(reader, std::io::BufWriter::new(writer),
                                      rpc_twoparty_capnp::Side::Server, Default::default());

        let rpc_system = RpcSystem::new(Box::new(network), Some(city_server.clone().client));
        current_thread::spawn(rpc_system.map_err(|e| println!("error: {:?}", e)));
        Ok(())
    });

    current_thread::block_on_all(done).unwrap();
}
