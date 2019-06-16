@0xcece72d96d1b463b;

interface CityServer {

    ping @0 () -> (pong :Text);

    newSession @1 (uid :Text) -> (session :Session);

    struct Session {
        sessionId @0 :Text;
        scene @1 :Scene;
    }

    struct Scene {
        type @0 :Text;

        union {
            echo @1 :EchoScene;
            demo @2 :DemoScene;
        }

        struct EchoScene {
            state @0 :Text;
        }

        struct DemoScene {
            state @0 :Text;
        }
    }
}
