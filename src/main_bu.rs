use ws::{listen, Sender, Handler, Result, Message};

#[derive(Copy, Clone)]
struct Loc {
    x: usize,
    y: usize
}

#[derive(Copy, Clone)]
struct GridWorld {
    size: (usize, usize),
    agent_loc: Loc
}

struct Agent<'a> {
    grid_world: &'a mut GridWorld,
    loc: Loc
}


#[derive(Copy, Clone, Debug)]
enum Move {
    UP, DOWN, LEFT, RIGHT, STAY
}

trait AgentMove {
    fn attempt_move(&mut self, mv: Move) -> bool;
}

trait ProcessMove {
    fn process_move(&mut self, mv: Move) -> bool;
}

impl<'a> AgentMove for Agent<'a> {
    fn attempt_move(&mut self, mv: Move) -> bool {
        // let grid_world = &mut self.grid_world;
        self.grid_world.process_move(mv)
    }
}

impl ProcessMove for GridWorld {
    fn process_move(&mut self, mv: Move) -> bool {
        match mv {
            Move::UP => if self.agent_y() < self.size.1 - 1 {
                self.agent_loc = Loc{x: self.agent_x(), y: self.agent_y() - 1};
                true
            } else {
                false
            },
            Move::DOWN => if self.agent_y() > 0 {
                self.agent_loc = Loc{x: self.agent_x(), y: self.agent_y() + 1};
                true
            } else {
                false
            },
            Move::LEFT => if self.agent_x() > 1 {
                self.agent_loc = Loc{x: self.agent_x() - 1, y: self.agent_y()};
                true
            } else {
                false
            },
            Move::RIGHT => if self.agent_x() < self.size.0 - 1 {
                println!("IN RIGHT");
                println!("{}", self);
                self.agent_loc = Loc{x: self.agent_x() + 1, y: self.agent_y() };
                println!("{}", self);
                true
            } else {
                false
            },
            _ => false
        }
    }
}

trait AgentLoc {
    fn agent_loc(&self) -> Loc;
    fn agent_x(&self) -> usize;
    fn agent_y(&self) -> usize;
}

impl AgentLoc for GridWorld {
    fn agent_loc(&self) -> Loc {
        self.agent_loc
    }

    fn agent_x(&self) -> usize {
        self.agent_loc.x
    }

    fn agent_y(&self) -> usize {
        self.agent_loc.y
    }
}

impl std::fmt::Display for GridWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut grid = vec![vec![0; self.size.0]; self.size.1];
        grid[self.agent_y()][self.agent_x()] = 1;
        println!("Agent loc: ({}, {})", self.agent_x(), self.agent_y());
        for row in grid {
            println!("{:?}", row);
        }
        write!(f, "")
    }
}


fn process_message(msg: ws::Message, grid_world: &mut GridWorld) -> String {
    let mv = match msg.as_text() {
        Ok("UP") => Move::UP,
        Ok("DOWN") => Move::DOWN,
        Ok("LEFT") => Move::LEFT,
        Ok("RIGHT") => Move::RIGHT,
        Ok(_) => Move::STAY,
        Err(_) => Move::STAY
    };
    if grid_world.process_move(mv) {
        format!("Success! Moved {:?}.\n {}", mv, grid_world)
    }
    else {
        String::from("Error, did not move.\n")
    }
}

struct Server {
    out: Sender,
    grid_world: GridWorld
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        move |msg| {
            self.out.send(msg)//process_message(msg, &mut self.grid_world))
        };
        Ok(())
    }
}

fn main() {

    let mut grid_world = GridWorld{
        size: (3, 4),
        agent_loc: Loc{x: 0, y: 0}
    };


    // let mut agent = Agent{grid_world: &grid_world, loc: Loc{x: 0, y: 0}};
    // for i in 0..5 {
    //     println!("{}", grid_world);
    //     grid_world.process_move(Move::RIGHT);
    // }

    listen("0.0.0.0:3012", |out| Server {out, grid_world});


  //  listen("0.0.0.0:3012", |out| {
	// move |msg| {
	//      out.send(msg)
	//}
  //});
}
