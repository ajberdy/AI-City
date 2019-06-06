
struct Loc {
    x: usize,
    y: usize
}

struct GridWorldServer {
    size: (usize, usize),
    agent_loc: Loc
}

impl Handler for GridWorldServer {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // schedule a timeout to send a ping every 5 seconds
        self.out.timeout(5_000, PING)?;
        // schedule a timeout to close the connection if there is no activity for 30 seconds
        self.out.timeout(30_000, EXPIRE)
    }
}


fn main() {
    let mut grid_world = GridWorldServer{
        size: (3, 4),
        agent_loc: Loc{
            x: 0,
            y: 0
        }
    };

    listen("0.0.0.0:3012", grid_world).unwrap();
}
