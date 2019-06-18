@0xcece72d96d1b463b;

interface CityServer {

    ping @0 () -> (pong :Text);

    getSession @1 (sessionArgs :SessionArgs) -> (session :Session);

    getState @2 (scene :Scene) -> (state :Text);

    struct SessionArgs {
        union {
            sessionId @0 :Text;
            newSessionArgs @1 :NewSessionArgs;
        }
    }

    struct NewSessionArgs {
        uid @0 :Text;
        scene @1 :SceneType;
    }

    struct Session {
        sessionId @0 :Text;
        scene @1 :Scene;
    }

    enum SceneType {
        ping @0;
        hiker @1;
        maze @2;
    }

    struct Scene {
       union {
            ping @0 :PingScene;
            hiker @1 :HikerScene;
            maze @2 :MazeScene;
        }

        struct PingScene {
            name @0 :Text;
            state @1 :Text;
        }

        struct HikerScene {
            name @0 :Text;
            state @1 :Text;
        }

        struct MazeScene {
            name @0 :Text;
            state @1 :Text;
        }
    }

    updateState @3 (sessionId :Text, arg: Text) -> (newState :Text);
}
