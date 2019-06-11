@0xcece72d96d1b463b;

interface CityServer {

    ping @0 () -> (pong :Text);

    newSession @1 (uid :Text) -> (session :Session);

    struct Session {
        sessionId @0 :Text;
    }
}
