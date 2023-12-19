pub mod game;


use crate::game::Manager;


#[tokio::main]
async fn main() {
    let mut manager = Manager::new_with_games(10);
    let game = manager.games.first_mut().unwrap();

    game.add_player("John");
    game.add_player("Jane");

    let mut table = game.run();
    table.add_one_point();

    // game.add_player("Jack");

    table.add_one_point();


    println!("Hello, world!");
}
