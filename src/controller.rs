use crate::model::GameModel;
use crate::view::GameView;
use crate::state::GameState;
use macroquad::prelude::*;

pub struct GameController {
    model: GameModel,
    view: GameView,
}

impl GameController {
    pub fn new() -> Self {
        Self {
            model: GameModel::new(),
            view: GameView::new(),
        }
    }

    pub async fn run(&mut self) {
        loop {
            let delta = get_frame_time();

            match self.model.state {
                GameState::Menu => {
                    if is_key_pressed(KeyCode::Space) {
                        self.model.state = GameState::Playing;
                    }
                }

                GameState::Playing => {
                    let mut dx = 0.0;
                    let mut dy = 0.0;

                    if is_key_down(KeyCode::Right) {
                        dx += 1.0;
                    }
                    if is_key_down(KeyCode::Left) {
                        dx -= 1.0;
                    }
                    if is_key_down(KeyCode::Down) {
                        dy += 1.0;
                    }
                    if is_key_down(KeyCode::Up) {
                        dy -= 1.0;
                    }

                    self.model.update_position(dx, dy, delta);

                    if self.model.player_x < 0.0
                        || self.model.player_x > screen_width()
                        || self.model.player_y < 0.0
                        || self.model.player_y > screen_height()
                    {
                        self.model.state = GameState::GameOver;
                    }
                }

                GameState::GameOver => {
                    if is_key_pressed(KeyCode::Enter) {
                        self.model = GameModel::new();
                    }
                }
            }

            self.view.render(&self.model);
            next_frame().await;
        }
    }
}
