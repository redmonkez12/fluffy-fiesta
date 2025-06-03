use macroquad::color::{WHITE, RED, GREEN, BLUE};
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{draw_texture_ex, draw_rectangle_lines, DrawTextureParams, Texture2D};

#[derive(PartialEq, Debug)]
pub enum CharacterDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum AnchorPoint {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

pub struct Animate {
    pub frame_size: Vec2,
    textures: Vec<Texture2D>,
    current_frame: u8,
    frame_time: f32,
    timer: f32,
    pub debug_mode: bool,
    pub looping: bool,
    finished: bool,
    frame_offsets: Vec<Vec2>,
    max_frame_size: Vec2,
}

impl Animate {
    pub fn new(textures: &Vec<Texture2D>, frame_time: f32) -> Self {
        let max_frame_size = Self::calculate_max_frame_size(textures);

        Self {
            textures: textures.clone(),
            current_frame: 0,
            frame_time,
            timer: 0.0,
            frame_size: Vec2::new(textures[0].width(), textures[0].height()),
            debug_mode: false,
            looping: true,
            finished: false,
            frame_offsets: vec![Vec2::ZERO; textures.len()],
            max_frame_size,
        }
    }

    pub fn new_normalized(textures: &Vec<Texture2D>, frame_time: f32) -> Self {
        let max_frame_size = Self::calculate_max_frame_size(textures);

        Self {
            textures: textures.clone(),
            current_frame: 0,
            frame_time,
            timer: 0.0,
            frame_size: max_frame_size,
            debug_mode: false,
            looping: true,
            finished: false,
            frame_offsets: vec![Vec2::ZERO; textures.len()],
            max_frame_size,
        }
    }

    pub fn new_with_offsets(textures: &Vec<Texture2D>, frame_time: f32, offsets: Vec<Vec2>) -> Self {
        let frame_size = if !textures.is_empty() {
            Vec2::new(textures[0].width(), textures[0].height())
        } else {
            Vec2::new(0.0, 0.0)
        };

        let max_frame_size = Self::calculate_max_frame_size(textures);
        let frame_offsets = if offsets.len() == textures.len() {
            offsets
        } else {
            vec![Vec2::ZERO; textures.len()]
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
            frame_offsets,
            max_frame_size,
        }
    }

    pub fn new_once(textures: &Vec<Texture2D>, frame_time: f32) -> Self {
        let mut animate = Self::new(textures, frame_time);
        animate.looping = false;
        animate
    }

    pub fn new_normalized_once(textures: &Vec<Texture2D>, frame_time: f32) -> Self {
        let mut animate = Self::new_normalized(textures, frame_time);
        animate.looping = false;
        animate
    }

    fn calculate_max_frame_size(textures: &Vec<Texture2D>) -> Vec2 {
        if textures.is_empty() {
            return Vec2::new(0.0, 0.0);
        }

        let max_width = textures.iter().map(|t| t.width()).fold(0.0, f32::max);
        let max_height = textures.iter().map(|t| t.height()).fold(0.0, f32::max);

        Vec2::new(max_width, max_height)
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

    pub fn get_max_frame_size(&self) -> Vec2 {
        self.max_frame_size
    }

    pub fn get_current_frame_size(&self) -> Vec2 {
        if self.textures.is_empty() || self.current_frame as usize >= self.textures.len() {
            return Vec2::ZERO;
        }

        let current_texture = &self.textures[self.current_frame as usize];
        Vec2::new(current_texture.width(), current_texture.height())
    }

    pub fn set_frame_offset(&mut self, frame_index: usize, offset: Vec2) {
        if frame_index < self.frame_offsets.len() {
            self.frame_offsets[frame_index] = offset;
        }
    }

    pub fn set_all_frame_offsets(&mut self, offsets: Vec<Vec2>) {
        if offsets.len() == self.textures.len() {
            self.frame_offsets = offsets;
        }
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

    pub fn draw_anchored(&mut self, position: Vec2, direction: &CharacterDirection, anchor: AnchorPoint) {
        if self.textures.is_empty() || self.current_frame as usize >= self.textures.len() {
            return;
        }

        let current_texture = &self.textures[self.current_frame as usize];
        let current_size = Vec2::new(current_texture.width(), current_texture.height());

        let offset = match anchor {
            AnchorPoint::TopLeft => Vec2::new(0.0, 0.0),
            AnchorPoint::TopCenter => Vec2::new(-current_size.x / 2.0, 0.0),
            AnchorPoint::TopRight => Vec2::new(-current_size.x, 0.0),
            AnchorPoint::CenterLeft => Vec2::new(0.0, -current_size.y / 2.0),
            AnchorPoint::Center => Vec2::new(-current_size.x / 2.0, -current_size.y / 2.0),
            AnchorPoint::CenterRight => Vec2::new(-current_size.x, -current_size.y / 2.0),
            AnchorPoint::BottomLeft => Vec2::new(0.0, -current_size.y),
            AnchorPoint::BottomCenter => Vec2::new(-current_size.x / 2.0, -current_size.y),
            AnchorPoint::BottomRight => Vec2::new(-current_size.x, -current_size.y),
        };

        let draw_pos = position + offset;

        let flip = match direction {
            CharacterDirection::Left => -1.0,
            CharacterDirection::Right => 1.0,
        };

        let final_draw_x = if *direction == CharacterDirection::Left {
            draw_pos.x + current_size.x
        } else {
            draw_pos.x
        };

        draw_texture_ex(
            current_texture,
            final_draw_x,
            draw_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(current_size.x * flip, current_size.y)),
                ..Default::default()
            },
        );

        if self.debug_mode {
            self.draw_debug_rectangles_anchored(position, draw_pos, current_size, direction, anchor);
        }
    }

    pub fn draw_centered(&mut self, position: Vec2, direction: &CharacterDirection) {
        if self.textures.is_empty() || self.current_frame as usize >= self.textures.len() {
            return;
        }

        let current_texture = &self.textures[self.current_frame as usize];
        let current_size = Vec2::new(current_texture.width(), current_texture.height());

        // Center the current frame within the max frame size
        let offset_x = (self.max_frame_size.x - current_size.x) / 2.0;
        let offset_y = (self.max_frame_size.y - current_size.y) / 2.0;

        let draw_pos = Vec2::new(position.x + offset_x, position.y + offset_y);

        let flip = match direction {
            CharacterDirection::Left => -1.0,
            CharacterDirection::Right => 1.0,
        };

        let final_draw_x = if *direction == CharacterDirection::Left {
            draw_pos.x + current_size.x
        } else {
            draw_pos.x
        };

        draw_texture_ex(
            current_texture,
            final_draw_x,
            draw_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(current_size.x * flip, current_size.y)),
                ..Default::default()
            },
        );

        if self.debug_mode {
            self.draw_debug_rectangles_centered(position, draw_pos, current_size, direction);
        }
    }

    pub fn draw_bottom_aligned(&mut self, ground_position: Vec2, direction: &CharacterDirection) {
        if self.textures.is_empty() || self.current_frame as usize >= self.textures.len() {
            return;
        }

        let current_texture = &self.textures[self.current_frame as usize];
        let current_size = Vec2::new(current_texture.width(), current_texture.height());

        // Draw from ground position up
        let draw_y = ground_position.y - current_size.y;
        let draw_pos = Vec2::new(ground_position.x, draw_y);

        let flip = match direction {
            CharacterDirection::Left => -1.0,
            CharacterDirection::Right => 1.0,
        };

        let final_draw_x = if *direction == CharacterDirection::Left {
            draw_pos.x + current_size.x
        } else {
            draw_pos.x
        };

        draw_texture_ex(
            current_texture,
            final_draw_x,
            draw_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(current_size.x * flip, current_size.y)),
                ..Default::default()
            },
        );

        if self.debug_mode {
            self.draw_debug_rectangles_bottom_aligned(ground_position, draw_pos, current_size, direction);
        }
    }

    pub fn draw_with_offsets(&mut self, position: Vec2, direction: &CharacterDirection) {
        if self.textures.is_empty() || self.current_frame as usize >= self.textures.len() {
            return;
        }

        let current_texture = &self.textures[self.current_frame as usize];
        let current_size = Vec2::new(current_texture.width(), current_texture.height());

        let frame_offset = self.frame_offsets.get(self.current_frame as usize)
            .copied()
            .unwrap_or(Vec2::ZERO);

        let adjusted_position = position + frame_offset;

        let flip = match direction {
            CharacterDirection::Left => -1.0,
            CharacterDirection::Right => 1.0,
        };

        let final_draw_x = if *direction == CharacterDirection::Left {
            adjusted_position.x + current_size.x
        } else {
            adjusted_position.x
        };

        draw_texture_ex(
            current_texture,
            final_draw_x,
            adjusted_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(current_size.x * flip, current_size.y)),
                ..Default::default()
            },
        );

        if self.debug_mode {
            self.draw_debug_rectangles_with_offsets(position, adjusted_position, current_size, direction, frame_offset);
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

    fn draw_debug_rectangles_anchored(&self, anchor_pos: Vec2, draw_pos: Vec2, current_size: Vec2, direction: &CharacterDirection, anchor: AnchorPoint) {
        let thickness = 2.0;

        // Draw actual frame rectangle (red)
        let final_draw_x = if *direction == CharacterDirection::Left {
            draw_pos.x + current_size.x
        } else {
            draw_pos.x
        };

        draw_rectangle_lines(
            final_draw_x,
            draw_pos.y,
            current_size.x.abs(),
            current_size.y,
            thickness,
            RED,
        );

        // Draw anchor point (blue)
        draw_rectangle_lines(
            anchor_pos.x - 2.0,
            anchor_pos.y - 2.0,
            4.0,
            4.0,
            1.0,
            BLUE,
        );
    }

    fn draw_debug_rectangles_centered(&self, position: Vec2, draw_pos: Vec2, current_size: Vec2, direction: &CharacterDirection) {
        let thickness = 2.0;

        // Draw max frame size area (green)
        draw_rectangle_lines(
            position.x,
            position.y,
            self.max_frame_size.x,
            self.max_frame_size.y,
            thickness,
            GREEN,
        );

        // Draw actual frame rectangle (red)
        let final_draw_x = if *direction == CharacterDirection::Left {
            draw_pos.x + current_size.x
        } else {
            draw_pos.x
        };

        draw_rectangle_lines(
            final_draw_x,
            draw_pos.y,
            current_size.x.abs(),
            current_size.y,
            thickness,
            RED,
        );

        // Draw center point (blue)
        let center_x = position.x + self.max_frame_size.x / 2.0;
        let center_y = position.y + self.max_frame_size.y / 2.0;
        draw_rectangle_lines(
            center_x - 2.0,
            center_y - 2.0,
            4.0,
            4.0,
            1.0,
            BLUE,
        );
    }

    fn draw_debug_rectangles_bottom_aligned(&self, ground_pos: Vec2, draw_pos: Vec2, current_size: Vec2, direction: &CharacterDirection) {
        let thickness = 2.0;

        // Draw ground line (green)
        draw_rectangle_lines(
            ground_pos.x - 10.0,
            ground_pos.y - 1.0,
            20.0,
            2.0,
            thickness,
            GREEN,
        );

        // Draw actual frame rectangle (red)
        let final_draw_x = if *direction == CharacterDirection::Left {
            draw_pos.x + current_size.x
        } else {
            draw_pos.x
        };

        draw_rectangle_lines(
            final_draw_x,
            draw_pos.y,
            current_size.x.abs(),
            current_size.y,
            thickness,
            RED,
        );

        // Draw ground anchor point (blue)
        draw_rectangle_lines(
            ground_pos.x - 2.0,
            ground_pos.y - 2.0,
            4.0,
            4.0,
            1.0,
            BLUE,
        );
    }

    fn draw_debug_rectangles_with_offsets(&self, original_pos: Vec2, adjusted_pos: Vec2, current_size: Vec2, direction: &CharacterDirection, offset: Vec2) {
        let thickness = 2.0;

        // Draw original position (green)
        draw_rectangle_lines(
            original_pos.x - 2.0,
            original_pos.y - 2.0,
            4.0,
            4.0,
            1.0,
            GREEN,
        );

        let final_draw_x = if *direction == CharacterDirection::Left {
            adjusted_pos.x + current_size.x
        } else {
            adjusted_pos.x
        };

        draw_rectangle_lines(
            final_draw_x,
            adjusted_pos.y,
            current_size.x.abs(),
            current_size.y,
            thickness,
            RED,
        );

        // Draw adjusted position (blue)
        draw_rectangle_lines(
            adjusted_pos.x - 2.0,
            adjusted_pos.y - 2.0,
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