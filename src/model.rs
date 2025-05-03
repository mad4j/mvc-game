use macroquad::prelude::*;
use crate::state::GameState;

pub struct GameModel {
    pub state: GameState,
    pub player_x: f32,
    pub player_y: f32,
    pub player_speed: f32,
}

impl GameModel {
    pub fn new() -> Self {
        Self {
            state: GameState::Menu,
            player_x: screen_width() / 2.0,
            player_y: screen_height() / 2.0,
            player_speed: 200.0,
        }
    }

    pub fn update_position(&mut self, dx: f32, dy: f32, delta: f32) {
        self.player_x += dx * self.player_speed * delta;
        self.player_y += dy * self.player_speed * delta;
    }
}
