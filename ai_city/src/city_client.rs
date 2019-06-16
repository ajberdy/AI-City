use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};

use crate::city_server_capnp::city_server;

use futures::Future;
use tokio::io::AsyncRead;
use tokio::runtime::current_thread::Runtime;


pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} client HOST:PORT", args[0]);
        return;
    }

    try_main(args).expect("failed");
}

fn ping(city_server: &city_server::Client,
        runtime: &mut Runtime)
        -> Result<String, ::capnp::Error> {
    let promise = city_server
        .ping_request()
        .send();
    let response = runtime.block_on(promise.promise)?;
    let pong = response.get()?.get_pong()?.to_string();
    Ok(pong)
}

#[derive(Display)]
#[display(fmt = "Session{{session_id: {}}}", session_id)]
struct Session {
    session_id: String
}

fn get_session(city_server: &city_server::Client,
               runtime: &mut Runtime,
               uid: &str)
        -> Result<Session, ::capnp::Error> {
    let mut request = city_server
        .get_session_request();
    request.get().set_uid(uid);
    let promise = request.send();
    let response = runtime.block_on(promise.promise)?;
    let session = response.get()?.get_session()?;
    let session_id = session.get_session_id()?.to_string();
    // println!("{}", Session{&session_id});
    Ok(Session{session_id})
}

fn try_main(args: Vec<String>) -> Result<(), ::capnp::Error> {
    use std::net::ToSocketAddrs;

    let mut runtime = Runtime::new().unwrap();

    let addr = args[2].to_socket_addrs()?.next().expect("could not parse address");
    let stream = runtime.block_on(::tokio::net::TcpStream::connect(&addr)).unwrap();
    stream.set_nodelay(true)?;
    let (reader, writer) = stream.split();

    let network =
        Box::new(twoparty::VatNetwork::new(reader, std::io::BufWriter::new(writer),
                                           rpc_twoparty_capnp::Side::Client,
                                           Default::default()));
    let mut rpc_system = RpcSystem::new(network, None);
    let city_server: city_server::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
    runtime.spawn(rpc_system.map_err(|_e| ()));

    let result = test_city_server(&city_server, &mut runtime)?;
    println!("{}", result);
    Ok(())
}


fn test_city_server(city_server: &city_server::Client, mut runtime: &mut Runtime)
                    -> Result<String, capnp::Error> {
    assert_eq!(ping(&city_server, &mut runtime)?, "pong");
    println!("pong test passed.");

    let uid = "704-655-624";
    let session = get_session(&city_server, &mut runtime, uid)?;
    assert_eq!(session.session_id, format!("session-{}", uid));
    println!("new session test passed.");

    Ok("tests passed".to_string())
}
