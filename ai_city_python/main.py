import capnp
import city_server_capnp
import sys


def main():
    host = sys.argv[1]
    client = capnp.TwoPartyClient(host)

    city_server = client.bootstrap().cast_as(city_server_capnp.CityServer)

    pong_promise = city_server.ping()
    pong = pong_promise.wait()
    print(pong.pong)

    uid = "704-655-624"
    session = city_server.getSession(uid).wait().session
    session_id = session.sessionId
    print(session)

    if len(sys.argv) > 2:
        arg = sys.argv[2]
        new_state = city_server.updateState(session_id, arg).wait().newState
        print(new_state)


if __name__ == "__main__":
    main()
