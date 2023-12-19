use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: String,
}

#[derive(Debug)]
pub struct Game<'a> {
    manager: Rc<RefCell<Manager<'a>>>,
    players: Vec<Player>,
}

#[derive(Default, Debug)]
pub struct Manager<'a> {
    pub games: Vec<Game<'a>>,
    tables: Vec<Table<'a>>,
}

impl Manager<'_> {
    pub fn new_with_games(number: usize) -> Self {
        let manager = Rc::new(RefCell::new(Self::default()));

        let tables = Vec::with_capacity(number);
        let games = (0..number).map(|_| Game::new(manager.clone())).collect();

        Self { games, tables }
    }
}

impl<'a> Game<'a> {
    pub fn new(manager: Rc<RefCell<Manager<'a>>>) -> Self {
        Game {
            manager,
            players: Vec::new(),
        }
    }

    pub fn run(&'a self) -> Table {
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

#[derive(Debug)]
pub struct Table<'a> {
    game: &'a Game<'a>,
    points: u8,
}

impl<'a> Table<'a> {
    pub fn new(game: &'a Game<'a>) -> Self {
        Table { game, points: 0 }
    }

    pub fn add_one_point(&mut self) {
        self.points += 1;
    }
}
