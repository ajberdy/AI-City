use capnp_rpc::pry;
use capnp;

use std::collections::HashMap;
use std::mem;

use crate::city_server_capnp::city_server;
use crate::city_server_architecture::{CityServer, Session, Scene, State};


impl city_server::Server for CityServer {
    fn ping(
        &mut self,
        _params: city_server::PingParams,
        mut results: city_server::PingResults
    ) -> capnp::capability::Promise<(), capnp::Error> {
        println!("pinged");
        results.get().set_pong("pong");
        capnp::capability::Promise::ok(())
    }

    fn get_session(
        &mut self,
        params: city_server::GetSessionParams,
        mut results: city_server::GetSessionResults
    ) -> capnp::capability::Promise<(), capnp::Error> {

        let args = pry!(params.get());
        let session_args = pry!(args.get_session_args());
        let session_args_type = pry!(session_args.which());

        let mut session = pry!(results.get().get_session());
        let session_id: String;

        match session_args_type {
            city_server::session_args::SessionId(session_id_promise) => {
                session_id = pry!(session_id_promise).to_string();
                match self.sessions.get(&session_id) {
                    Some(session) => {
                        println!("found session: {}", session_id);
                    },
                    None => {
                        println!("no session found");
                    }
                }
            },
            city_server::session_args::NewSessionArgs(new_session_args_promise) => {
                let new_session_args = pry!(new_session_args_promise);
                let uid = pry!(new_session_args.get_uid());
                let type_str;
                let scene;
                match pry!(new_session_args.get_scene()) {
                    city_server::SceneType::Ping => {
                        println!("ping!");
                        type_str = "ping";
                        scene = Scene::PingScene;
                    },
                    city_server::SceneType::Hiker => {
                        println!("hiker!");
                        type_str = "hiker";
                        scene = Scene::HikerScene;
                    },
                    city_server::SceneType::Maze => {
                        println!("maze!");
                        type_str = "maze";
                        scene = Scene::MazeScene{
                            grid: vec![vec![1, 1, 1, 1, 1],
                                       vec![1, 0, 1, 0, 1],
                                       vec![1, 0, 0, 0, 1],
                                       vec![1, 0, 1, 0, 1],
                                       vec![1, 1, 1, 1, 1]],
                            agent_loc: (1, 1)
                        };
                    }
                }
                session_id = format!("session-{}-{}", uid, type_str);
                println!("created new session: {}", session_id);
                self.sessions.insert(session_id.clone(),
                                     Session{
                                         session_id: session_id.clone(),
                                         scene
                                     });
            }
        }

        session.set_session_id(&session_id);
        let mut capnp_scene = pry!(session.get_scene());
        capnp_scene.set_name(scene.get_name());

        // println!("{}", session_args_type);
        // match session_args.which() {
        //     city_server::new_session_args::SessionId(session_id) =>
        //         println!("haha"),
        //     _ => println!("uh oh")
        // }
        /*
        let args = pry!(params.get());
        let uid = pry!(args.get_uid());

        let session_id = format!("session-{}", uid);

        match self.sessions.get(&session_id) {
            Some(session) => 
        }

        let mut session = pry!(results.get().get_session());
        session.set_session_id(&session_id.clone());
        let scene = pry!(session.get_scene());

        let maze_scene = Scene::MazeScene{
            grid: vec![vec![1, 1, 1, 1, 1],
                       vec![1, 0, 1, 0, 1],
                       vec![1, 0, 0, 0, 1],
                       vec![1, 0, 1, 0, 1],
                       vec![1, 1, 1, 1, 1]],
            agent_loc: (1, 1)
        };
        let session = Session{
            session_id,
            scene: maze_scene
        };
        */
        capnp::capability::Promise::ok(())
    }

}







/*

pub struct CityServerImpl {
    sessions: HashMap<String, Session>
}

impl CityServerImpl {
    pub fn new() -> CityServerImpl {
        CityServerImpl {
            sessions: HashMap::new()
        }
    }
}


struct Session {
    session_id: String,
    scene: Scene
}

struct Scene{
    state: String
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

    fn get_session(
        &mut self,
        params: city_server::GetSessionParams,
        mut results: city_server::GetSessionResults
    ) -> capnp::capability::Promise<(), capnp::Error> {

        let args = pry!(params.get());
        let uid = pry!(args.get_uid());
        let session_type = pry!(args.get_type());

        let session_type_str = match session_type {
            city_server::SessionType::Echo => "echo",
            city_server::SessionType::Demo => "demo"
        };
        let session_id = format!("session-{}-{}", uid, session_type_str);
        println!("{}", session_id);

        let mut session = pry!(results.get().get_session());
        session.set_session_id(&session_id.clone());
        let scene = pry!(session.get_scene());
        match session_type {
            city_server::SessionType::Echo => {
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
                        println!("created new session")
                    }
                }
            }
            city_server::SessionType::Demo => {
                let mut demo = scene.init_demo();
                match self.sessions.get(&session_id) {
                    Some(session) => {
                        println!("found {}", session.session_id);
                        demo.set_state(&session.scene.state);
                    },
                    None => {
                        let scene = Scene{
                            state: "start demo state".to_string()
                        };
                        demo.set_state(&scene.state);
                        self.sessions.insert(session_id.clone(),
                                             Session{session_id, scene});
                        println!("created new session")
                    }
                }
            }
        }
        capnp::capability::Promise::ok(())
    }

    fn update_state(
        &mut self,
        params: city_server::UpdateStateParams,
        mut results: city_server::UpdateStateResults
    ) -> capnp::capability::Promise<(), capnp::Error> {
        let args = pry!(params.get());
        let session_id = pry!(args.get_session_id());
        let arg = pry!(args.get_arg());

        match self.sessions.get_mut(session_id) {
            Some(session) => {
                println!("old state: {}", session.scene.state);
                mem::replace(&mut session.scene.state, arg.to_string());
                results.get().set_new_state(arg);
                println!("new state: {}", session.scene.state);
                capnp::capability::Promise::ok(())
            }
            None => capnp::capability::Promise::err(
                capnp::Error::failed("tried updating state with an invalid session id".to_string()))
        }
    }

}

*/
