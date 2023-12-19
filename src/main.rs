pub mod game;


use crate::game::Manager;


#[tokio::main]
async fn main() {
    let manager = Manager::new_with_games(10);


    println!("Hello, world!");
}
