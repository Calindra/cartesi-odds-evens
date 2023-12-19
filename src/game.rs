use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Game {
    players: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    games: Vec<Game>,
    tables: Vec<Table>,
}

impl Manager {
    pub fn new_with_games(number: usize) -> Self {
        let tables = Vec::with_capacity(number);
        let games = (0..number).map(|_| Game::default()).collect();

        Self { games, tables }
    }
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
