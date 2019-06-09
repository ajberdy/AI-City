import capnp
import grid_world_capnp
import sys


def main():
    host = sys.argv[1]
    client = capnp.TwoPartyClient(host)

    grid_world = client.bootstrap().cast_as(grid_world_capnp.GridWorld)

    pong_promise = grid_world.ping()
    pong = pong_promise.wait()
    print(pong.pong)


if __name__ == "__main__":
    main()
