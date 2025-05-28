use macroquad::color::WHITE;
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};

#[derive(PartialEq)]
pub enum CharacterDirection {
    Left,
    Right,
}

pub struct Animate {
    pub frame_size: Vec2,
    texture: Texture2D,
    total_frames: u8,
    current_frame: u8,
    frame_time: f32,
    timer: f32,
}

impl Animate {
    pub fn new(texture: &Texture2D, total_frames: u8, frame_time: f32) -> Self {
        let frame_width = texture.width() / total_frames as f32;
        let frame_height = texture.height();

        Self {
            texture: texture.clone(),
            total_frames,
            current_frame: 0,
            frame_time,
            timer: 0.0,
            frame_size: Vec2::new(frame_width, frame_height),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        if self.timer >= self.frame_time {
            self.timer = 0.0;
            self.current_frame += 1;
            if self.current_frame >= self.total_frames {
                self.current_frame = 0;
            }
        }
    }

    pub fn draw(&mut self, position: Vec2, direction: &CharacterDirection) {
        let flip = match direction {
            CharacterDirection::Left => -1.0,
            CharacterDirection::Right => 1.0,
        };

        let rect = Rect::new(
            self.current_frame as f32 * self.frame_size.x,
            0.0,
            self.frame_size.x,
            self.frame_size.y,
        );

        let draw_x = if *direction == CharacterDirection::Left {
            position.x + self.frame_size.x
        } else {
            position.x
        };

        draw_texture_ex(
            &self.texture,
            draw_x,
            position.y,
            WHITE,
            DrawTextureParams {
                source: Some(rect),
                dest_size: Some(Vec2::new(self.frame_size.x * flip, self.frame_size.y)),
                ..Default::default()
            },
        );
    }
}