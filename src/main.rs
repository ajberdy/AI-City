use ws::{listen, Handler, Handshake, Result, Sender, Message, CloseCode};

struct Loc {
    x: usize,
    y: usize
}

struct GridWorldServer {
    out: Sender,
    size: (usize, usize),
    agent_loc: Loc
}

impl std::fmt::Debug for GridWorldServer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut grid = vec![vec![0; self.size.0]; self.size.1];
        grid[self.agent_loc.y][self.agent_loc.x] = 1;
        let mut grid_world_str = format!("Agent loc: ({}, {})", self.agent_loc.x, self.agent_loc.y);
        for row in grid {
            grid_world_str = format!("{}\n{:?}", grid_world_str, row);
        }
        write!(f, "{}", grid_world_str)
    }
}

trait GridWorld {
    fn process_move(&mut self, mv: &str) -> bool;
}

impl GridWorld for GridWorldServer {
    fn process_move(&mut self, mv: &str) -> bool {
        match mv {
            "UP" => if self.agent_loc.y > 0 {
                self.agent_loc.y -= 1;
                true
            } else {
                false
            },
            "DOWN" => if self.agent_loc.y < self.size.1 - 1 {
                self.agent_loc.y += 1;
                true
            } else {
                false
            },
            "LEFT" => if self.agent_loc.x > 0 {
                self.agent_loc.x -= 1;
                true
            } else {
                false
            },
            "RIGHT" => if self.agent_loc.x < self.size.0 - 1 {
                self.agent_loc.x += 1;
                true
            } else {
                false
            }
            _ => false
        }
    }
}

impl Handler for GridWorldServer {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("on open");
        self.out.send(format!("Initial grid_world state:\n{:?}", self))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("on message: {}", msg);

        let move_str = msg.as_text().unwrap();
        if self.process_move(move_str) {
            // println!("{:?}", self)
            self.out.send(format!("Moved {}\n{:?}", move_str, self))
        } else {
            self.out.send(format!("Could not move {}\n{:?}", move_str, self))
        }
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Connection closed ({:?}) {}", code, reason);

        // self.out.shutdown().unwrap();
    }
}


fn main() {
    listen("0.0.0.0:3012", |out| GridWorldServer {
        out,
        size: (3, 4),
        agent_loc: Loc{
            x: 0,
            y: 0
        }
    }).unwrap();
}
