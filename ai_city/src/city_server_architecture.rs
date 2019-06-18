use std::collections::HashMap;


pub struct CityServer {
    pub sessions: HashMap<String, Session>
}

impl CityServer {
    pub fn new() -> CityServer {
        CityServer{
            sessions: HashMap::new()
        }
    }
}

pub struct Session{
    pub session_id: String,
    pub scene: Scene
}

pub enum Scene {
    PingScene,
    HikerScene,
    MazeScene {
        grid: Vec<Vec<u32>>,
        agent_loc: (usize, usize)
    }
}

pub enum State<'a> {
    PingState,
    HikerState,
    MazeState {
        grid_ref: &'a Vec<Vec<u32>>,
        agent_loc: (usize, usize)
    }
}

impl Scene {
    fn get_name(&self) -> String {
        match self {
            Scene::PingScene => "PingScene".to_string(),
            Scene::HikerScene => "HikerScene".to_string(),
            Scene::MazeScene{
                grid: _,
                agent_loc: _
            } => "MazeScene".to_string()
        }
    }

    fn get_state<'a>(&'a self) -> State<'a> {
        match self {
            Scene::PingScene => State::PingState,
            Scene::HikerScene => State::HikerState,
            Scene::MazeScene{
                grid,
                agent_loc
            } => State::MazeState{
                grid_ref: grid,
                agent_loc: *agent_loc
            }
        }
    }
}

impl<'a> std::fmt::Display for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::PingState => write!(f, "{}", "pong"),
            State::HikerState => write!(f, "{}", 42),
            State::MazeState{
                grid_ref,
                agent_loc
            } => {
                let mut maze_str = "".to_string();
                for row in *grid_ref {
                    maze_str += &format!("{:?}\n", row);
                }
                write!(f, "{}", maze_str)
            }
        }
    }
}


pub fn test_architecture() {
    let city_server = CityServer::new();

    let maze_scene = Scene::MazeScene{
        grid: vec![vec![1, 1, 1, 1, 1],
                   vec![1, 0, 1, 0, 1],
                   vec![1, 0, 0, 0, 1],
                   vec![1, 0, 1, 0, 1],
                   vec![1, 1, 1, 1, 1]],
        agent_loc: (1, 1)
    };
    println!("{}", maze_scene.get_state());


    let ping_scene = Scene::PingScene;
    println!("{}", ping_scene.get_state());

    let hiker_scene = Scene::HikerScene;
    println!("{}", hiker_scene.get_state());

}
