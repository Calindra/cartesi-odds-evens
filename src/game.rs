use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Game {
    players: Vec<Player>,
}

#[derive(Debug, Serialize)]
pub struct Manager<'a> {
    pub games: Vec<Game>,
    tables: Vec<Table<'a>>,
}

impl Manager<'_> {
    pub fn new_with_games(number: usize) -> Self {
        let tables = Vec::with_capacity(number);
        let games = (0..number).map(|_| Game::default()).collect();

        Self { games, tables }
    }
}

impl Game {
    pub fn run(&self) -> Table {
        println!("Game is running");
        Table::new(self)
    }

    pub fn add_player(&mut self, player: &str) {
        let player = Player {
            name: player.to_string(),
        };

        self.players.push(player);
    }
}

#[derive(Debug, Serialize)]
pub struct Table<'a> {
    game: &'a Game,
    points: u8,
}

impl<'a> Table<'a> {
    pub fn new(game: &'a Game) -> Self {
        Table { game, points: 0 }
    }

    pub fn add_one_point(&mut self) {
        self.points += 1;
    }
}
