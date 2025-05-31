use macroquad::color::{WHITE, RED, GREEN, BLUE};
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{draw_texture_ex, draw_rectangle_lines, DrawTextureParams, Texture2D};

#[derive(PartialEq, Debug)]
pub enum CharacterDirection {
    Left,
    Right,
}

pub struct Animate {
    pub frame_size: Vec2,
    textures: Vec<Texture2D>,  // Changed: Now stores individual frame textures
    current_frame: u8,
    frame_time: f32,
    timer: f32,
    pub debug_mode: bool,
    pub looping: bool,
    finished: bool,
}

impl Animate {
    // Updated constructor for individual frame textures
    pub fn new(textures: &Vec<Texture2D>, frame_time: f32) -> Self {
        let frame_size = if !textures.is_empty() {
            Vec2::new(textures[0].width(), textures[0].height())
        } else {
            Vec2::new(0.0, 0.0)
        };

        Self {
            textures: textures.clone(),
            current_frame: 0,
            frame_time,
            timer: 0.0,
            frame_size,
            debug_mode: false,
            looping: true,
            finished: false,
        }
    }

    // Create a non-looping animation
    pub fn new_once(textures: &Vec<Texture2D>, frame_time: f32) -> Self {
        let mut animate = Self::new(textures, frame_time);
        animate.looping = false;
        animate
    }

    pub fn update(&mut self, dt: f32) {
        if self.finished && !self.looping {
            return;
        }

        self.timer += dt;
        if self.timer >= self.frame_time {
            self.timer = 0.0;
            self.current_frame += 1;

            let total_frames = self.textures.len() as u8;
            if self.current_frame >= total_frames {
                if self.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = total_frames - 1;
                    self.finished = true;
                }
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.timer = 0.0;
        self.finished = false;
    }

    pub fn set_looping(&mut self, looping: bool) {
        self.looping = looping;
        if looping {
            self.finished = false;
        }
    }

    pub fn get_current_frame(&self) -> u8 {
        self.current_frame
    }

    pub fn get_total_frames(&self) -> u8 {
        self.textures.len() as u8
    }

    pub fn is_looping(&self) -> bool {
        self.looping
    }

    pub fn draw(&mut self, position: Vec2, direction: &CharacterDirection) {
        if self.textures.is_empty() || self.current_frame as usize >= self.textures.len() {
            return;
        }

        let current_texture = &self.textures[self.current_frame as usize];

        let flip = match direction {
            CharacterDirection::Left => -1.0,
            CharacterDirection::Right => 1.0,
        };

        let draw_x = if *direction == CharacterDirection::Left {
            position.x + self.frame_size.x
        } else {
            position.x
        };

        draw_texture_ex(
            current_texture,
            draw_x,
            position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.frame_size.x * flip, self.frame_size.y)),
                ..Default::default()
            },
        );

        if self.debug_mode {
            self.draw_debug_rectangles(position, direction, draw_x);
        }
    }

    pub fn draw_rotated(&self, position: Vec2, direction: &CharacterDirection, angle: f32) {
        if self.textures.is_empty() || self.current_frame as usize >= self.textures.len() {
            return;
        }

        let current_texture = &self.textures[self.current_frame as usize];

        let flip_x = match direction {
            CharacterDirection::Left => true,
            CharacterDirection::Right => false,
        };

        let draw_position = Vec2::new(
            position.x - self.frame_size.x / 2.0,
            position.y - self.frame_size.y / 2.0,
        );

        draw_texture_ex(
            current_texture,
            draw_position.x,
            draw_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.frame_size),
                flip_y: flip_x,
                rotation: angle,
                pivot: None,
                ..Default::default()
            },
        );

        if self.debug_mode {
            self.draw_debug_rectangles_rotated(position, draw_position, angle);
        }
    }

    fn draw_debug_rectangles(&self, position: Vec2, direction: &CharacterDirection, draw_x: f32) {
        let thickness = 2.0;

        draw_rectangle_lines(
            draw_x,
            position.y,
            self.frame_size.x.abs(),
            self.frame_size.y,
            thickness,
            RED,
        );

        draw_rectangle_lines(
            position.x,
            position.y,
            self.frame_size.x,
            self.frame_size.y,
            thickness,
            GREEN,
        );

        let center_x = match direction {
            CharacterDirection::Left => draw_x + self.frame_size.x / 2.0,
            CharacterDirection::Right => draw_x + self.frame_size.x / 2.0,
        };
        let center_y = position.y + self.frame_size.y / 2.0;

        draw_rectangle_lines(
            center_x - 2.0,
            center_y - 2.0,
            4.0,
            4.0,
            1.0,
            BLUE,
        );
    }

    fn draw_debug_rectangles_rotated(&self, center_position: Vec2, draw_position: Vec2, _angle: f32) {
        let thickness = 2.0;

        draw_rectangle_lines(
            draw_position.x,
            draw_position.y,
            self.frame_size.x,
            self.frame_size.y,
            thickness,
            RED,
        );

        draw_rectangle_lines(
            center_position.x - 2.0,
            center_position.y - 2.0,
            4.0,
            4.0,
            1.0,
            BLUE,
        );

        draw_rectangle_lines(
            draw_position.x - 1.0,
            draw_position.y - 1.0,
            2.0,
            2.0,
            1.0,
            GREEN,
        );
    }

    pub fn enable_debug(&mut self) {
        self.debug_mode = true;
    }

    pub fn disable_debug(&mut self) {
        self.debug_mode = false;
    }

    pub fn toggle_debug(&mut self) {
        self.debug_mode = !self.debug_mode;
    }

    pub fn set_debug(&mut self, enabled: bool) {
        self.debug_mode = enabled;
    }
}