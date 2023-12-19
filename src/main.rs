pub mod game;


use crate::game::Manager;


#[tokio::main]
async fn main() {
    let mut manager = Manager::new_with_games(10);
    let game = manager.games.first_mut().unwrap();

    game.add_player("John").unwrap();
    game.add_player("Jane").unwrap();

    let mut table = game.run().unwrap();
    table.add_one_point();
    table.add_one_point();

    // game.add_player("Jack");


    println!("Hello, world!");
}
