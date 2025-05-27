use crate::constants::{FRAME_IDLE_COUNT, FRAME_IDLE_DURATION, SCREEN_HEIGHT, SCREEN_WIDTH, WALK_FRAME_COUNT, WALK_FRAME_DURATION, PLAYER_SPEED};
use macroquad::color::WHITE;
use macroquad::input::{KeyCode, is_key_down};
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{DrawTextureParams, draw_texture_ex, get_frame_time};
use macroquad::texture::Texture2D;

#[derive(PartialEq)]
enum CharacterState {
    Idle,
    Walk,
}

#[derive(PartialEq)]
enum CharacterDirection {
    Left,
    Right,
}

pub struct Character {
    pub pos_x: f32,
    pub pos_y: f32,
    pub idle_texture: Texture2D,
    pub walk_texture: Texture2D,
    pub walk_frame_width: f32,
    pub frame_height: f32,
    pub idle_frame_width: f32,
    pub idle_current_frame: usize,
    pub walk_current_frame: usize,
    pub idle_frame_timer: f32,
    pub walk_frame_timer: f32,
    direction: CharacterDirection,
    state: CharacterState,
}

impl Character {
    pub fn new(idle_texture: &Texture2D, walk_texture: &Texture2D) -> Self {
        Self {
            pos_x: 0.0,
            pos_y: SCREEN_HEIGHT - idle_texture.height() - 10.0,
            idle_texture: idle_texture.clone(),
            walk_texture: walk_texture.clone(),
            direction: CharacterDirection::Right,
            state: CharacterState::Idle,
            frame_height: idle_texture.height(),
            idle_frame_width: idle_texture.width() / FRAME_IDLE_COUNT as f32,
            walk_frame_width: walk_texture.width() / WALK_FRAME_COUNT as f32,
            idle_current_frame: 0,
            walk_current_frame: 0,
            walk_frame_timer: 0.0,
            idle_frame_timer: 0.0,
        }
    }

    pub fn handle_keys(&mut self) {
        let mut new_pos_x = self.pos_x;
        let mut should_walk = false;
        let dt = get_frame_time();

        if is_key_down(KeyCode::Left) {
            new_pos_x -= PLAYER_SPEED * dt;
            self.direction = CharacterDirection::Left;
            should_walk = true;
        } else if is_key_down(KeyCode::Right) {
            new_pos_x += PLAYER_SPEED * dt;
            self.direction = CharacterDirection::Right;
            should_walk = true;
        } else {
            self.state = CharacterState::Idle;
        }

        let current_frame_width = match self.state {
            CharacterState::Idle => self.idle_frame_width,
            CharacterState::Walk => self.walk_frame_width,
        };

        if new_pos_x >= 0.0 && new_pos_x + current_frame_width <= SCREEN_WIDTH {
            self.pos_x = new_pos_x;

            if should_walk {
                self.state = CharacterState::Walk;
            } else {
                self.state = CharacterState::Idle;
            }
        }

        if !should_walk {
            self.state = CharacterState::Idle;
        }
    }

    pub fn update(&mut self) {
        self.idle_frame_timer += get_frame_time();
        if self.idle_frame_timer > FRAME_IDLE_DURATION {
            self.idle_frame_timer = 0.0;
            self.idle_current_frame = (self.idle_current_frame + 1) % FRAME_IDLE_COUNT;
        }

        if self.state == CharacterState::Walk {
            self.walk_frame_timer += get_frame_time();
            if self.walk_frame_timer > WALK_FRAME_DURATION {
                self.walk_frame_timer = 0.0;
                self.walk_current_frame = (self.walk_current_frame + 1) % WALK_FRAME_COUNT;
            }
        }
    }

    pub fn draw(&self) {
        let current_frame_width = match self.state {
            CharacterState::Idle => self.idle_frame_width,
            CharacterState::Walk => self.walk_frame_width,
        };

        let source_rect = match self.state {
            CharacterState::Idle => Rect::new(
                self.idle_current_frame as f32 * current_frame_width,
                0.0,
                current_frame_width,
                self.frame_height,
            ),  
            CharacterState::Walk => Rect::new(
                self.walk_current_frame as f32 * current_frame_width,
                0.0,
                current_frame_width,
                self.frame_height,
            ),
        };

        let flip = match self.direction {
            CharacterDirection::Left => -1.0,
            CharacterDirection::Right => 1.0,
        };

        let texture = match self.state {
            CharacterState::Idle => &self.idle_texture,
            CharacterState::Walk => &self.walk_texture,
        };

        let draw_x = if flip < 0.0 {
            self.pos_x + current_frame_width
        } else {
            self.pos_x
        };

        draw_texture_ex(
            &texture,
            draw_x,
            self.pos_y,
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                dest_size: Some(Vec2::new(current_frame_width * flip, self.frame_height)),
                ..Default::default()
            },
        );
    }
}
