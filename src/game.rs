use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: String
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Game {
    players: Vec<Player>,
}

impl Game {
    pub fn run(self) -> Table {
        println!("Game is running");
        Table::new(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    game: Game,
}

impl Table {
    pub fn new(game: Game) -> Self {
        Table { game }
    }
}