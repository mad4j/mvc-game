mod controller;
mod model;
mod view;
mod state;

use controller::GameController;

#[macroquad::main("MVC Game with State")]
async fn main() {
    let mut controller = GameController::new();
    controller.run().await;
}
