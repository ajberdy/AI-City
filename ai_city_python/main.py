import capnp
import city_server_capnp
import sys

from typing import Union
from enum import Enum


# DEMO_SCENE = city_server_capnp.CityServer.SessionType.demo
# ECHO_SCENE = city_server_capnp.CityServer.SessionType.echo

class SceneType:
    ping = 0
    hiker = 1
    maze = 2


class CityServer:

    def __init__(self, city_server_rpc):
        self._city_server = city_server_rpc
        self._sessions = {}

    def ping(self):
        """
        Pings city server to get a response

        :return: pong
        """
        pong_promise = self._city_server.ping()
        pong_response = pong_promise.wait()
        pong = pong_response.pong
        return pong

    def new_session(self, uid: str, scene: Union[str, int]):
        """
        Creates a new session using uid with scene type specified by scene. If a session with the
        same uid and scene type already exists, it will be reset.

        :param uid:
        :param scene:
        :return: new session
        """
        new_session_request = self._city_server.getSession_request({
            'newSessionArgs': {
                'uid': uid,
                'scene': scene
            }
        })
        new_session_promise = new_session_request.send()
        new_session = new_session_promise.wait().session
        print(new_session)
        print(new_session.sessionId)
        print(new_session.scene)
        session = Session(new_session)

        self._sessions[session.session_id] = session

        return session

    def get_session(self, session_id: str):
        """
        Get existing session by session id.

        :param session_id:
        :return: existing session
        """


class Session:

    def __init__(self, session_rpc):
        self.session_id = session_rpc.sessionId
        self.scene = Scene(session_rpc.scene)

        print(self.scene)

    def __str__(self):
        return f"<Session({self.session_id}): {self.scene.name}>"


class Scene:

    def __init__(self, scene_rpc):
        self.name = scene_rpc.name



def main():
    host = sys.argv[1]
    client = capnp.TwoPartyClient(host)

    city_server_rpc = client.bootstrap().cast_as(city_server_capnp.CityServer)
    city_server = CityServer(city_server_rpc)

    print(city_server.ping())

    uid = "555-555-555"
    ping_session = city_server.new_session(uid, SceneType.ping)
    print(ping_session)

    exit(0)


    # session_args = city_server_capnp.CityServer.SessionArgs.NewSessionArgs.init()
    new_session_request = city_server.getSession_request()
    new_session_args = new_session_request.sessionArgs.init("newSessionArgs")
    new_session_args.uid = "555-555-555"
    new_session_args.scene = SceneType.hiker
    new_session_promise = new_session_request.send()
    new_session = new_session_promise.wait()
    print("new session: ", new_session)

    existing_session_request = city_server.getSession_request()
    existing_session_args = existing_session_request.sessionArgs.sessionId = "session-555-555-555-ping"
    existing_session_promise = existing_session_request.send()
    existing_session = existing_session_promise.wait()
    print(existing_session)

    exit(0)


if __name__ == "__main__":
    main()
