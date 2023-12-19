use std::{cell::RefCell, error::Error, rc::Rc};

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

#[derive(Debug)]
pub struct Manager<'a> {
    pub games: Vec<Game<'a>>,
    tables: Vec<Table<'a>>,
}

impl Manager<'_> {
    pub fn new_with_games(number: usize) -> Self {
        let manager = Rc::new(RefCell::new(Self {
            tables: Vec::with_capacity(number),
            games: Vec::with_capacity(number),
        }));

        let games = (0..number).map(|_| Game::new(manager.clone())).collect();

        manager.borrow_mut().games = games;

        // Downgrade the Rc<RefCell<Manager>> to Manager
        let inside = Rc::into_inner(manager).unwrap();
        let manager = RefCell::into_inner(inside);
        manager
    }
}

impl<'a> Game<'a> {
    pub fn new(manager: Rc<RefCell<Manager<'a>>>) -> Self {
        Game {
            manager,
            players: Vec::new(),
        }
    }

    pub fn run(&'a self) -> Result<Table, Box<dyn Error>> {
        if self.players.len() < 2 {
            return Err("Not enough players".into());
        }

        println!("Game is running");
        Ok(Table::new(self))
    }

    pub fn add_player(&mut self, player: &str) -> Result<(), Box<dyn Error>> {
        if self.players.len() >= 2 {
            return Err("Game is full".into());
        }

        let player = Player {
            name: player.to_string(),
        };

        self.players.push(player);

        Ok(())
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
