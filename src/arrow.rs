use macroquad::color::WHITE;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};

pub struct Arrow {
    pub pos: Vec2,
    pub velocity: Vec2,
    texture: Texture2D,
}

impl Arrow {
    pub fn new(pos: Vec2, velocity: Vec2, texture: Texture2D) -> Self {
        Arrow { pos, velocity, texture }
    }

    pub fn update(&mut self, dt: f32) {
        let gravity = vec2(0.0, 200.0);

        self.velocity += gravity * dt;

        self.pos += self.velocity * dt;
    }

    pub fn draw(&self) {
        let angle = self.velocity.y.atan2(self.velocity.x);

        draw_texture_ex(
            &self.texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                rotation: angle,
                dest_size: Some(vec2(32.0, 8.0)),
                ..Default::default()
            },
        );
    }
}