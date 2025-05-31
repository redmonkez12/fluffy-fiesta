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
    texture: Texture2D,
    total_frames: u8,
    current_frame: u8,
    frame_time: f32,
    timer: f32,
    pub debug_mode: bool,
    pub looping: bool,          // Whether animation should loop
    finished: bool,         // Whether animation has completed
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
            debug_mode: false,
            looping: true,      // Default to looping
            finished: false,
        }
    }

    // Create a non-looping animation (useful for hit/death animations)
    pub fn new_once(texture: &Texture2D, total_frames: u8, frame_time: f32) -> Self {
        let mut animate = Self::new(texture, total_frames, frame_time);
        animate.looping = false;
        animate
    }

    pub fn update(&mut self, dt: f32) {
        // Don't update if animation is finished and not looping
        if self.finished && !self.looping {
            return;
        }

        self.timer += dt;
        if self.timer >= self.frame_time {
            self.timer = 0.0;
            self.current_frame += 1;

            if self.current_frame >= self.total_frames {
                if self.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.total_frames - 1; // Stay on last frame
                    self.finished = true;
                }
            }
        }
    }

    // Check if animation has completed (for non-looping animations)
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    // Reset animation to start
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.timer = 0.0;
        self.finished = false;
    }

    // Set whether animation should loop
    pub fn set_looping(&mut self, looping: bool) {
        self.looping = looping;
        if looping {
            self.finished = false; // Re-enable if switching back to looping
        }
    }

    // Get current frame number
    pub fn get_current_frame(&self) -> u8 {
        self.current_frame
    }

    // Get total frames
    pub fn get_total_frames(&self) -> u8 {
        self.total_frames
    }

    // Check if animation is looping
    pub fn is_looping(&self) -> bool {
        self.looping
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

        if self.debug_mode {
            self.draw_debug_rectangles(position, direction, draw_x);
        }
    }

    pub fn draw_rotated(&self, position: Vec2, direction: &CharacterDirection, angle: f32) {
        let flip_x = match direction {
            CharacterDirection::Left => true,
            CharacterDirection::Right => false,
        };

        let draw_position = Vec2::new(
            position.x - self.frame_size.x / 2.0,
            position.y - self.frame_size.y / 2.0,
        );

        draw_texture_ex(
            &self.texture,
            draw_position.x,
            draw_position.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.get_current_frame_rect()),
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

    fn get_current_frame_rect(&self) -> Rect {
        Rect::new(
            self.current_frame as f32 * self.frame_size.x,
            0.0,
            self.frame_size.x,
            self.frame_size.y,
        )
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