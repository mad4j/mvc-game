use macroquad::prelude::*;
use crate::model::GameModel;
use crate::state::GameState;

pub struct GameView;

impl GameView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, model: &GameModel) {
        clear_background(BLACK);

        match model.state {
            GameState::Menu => {
                draw_text("Premi SPAZIO per iniziare", 100.0, 100.0, 40.0, WHITE);
            }
            GameState::Playing => {
                draw_circle(model.player_x, model.player_y, 20.0, YELLOW);
            }
            GameState::GameOver => {
                draw_text("Game Over!", 100.0, 100.0, 40.0, RED);
                draw_text("Premi INVIO per tornare al menu", 100.0, 150.0, 30.0, WHITE);
            }
        }
    }
}
