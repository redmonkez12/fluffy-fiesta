use crate::animate::{Animate, CharacterDirection};
use crate::constants::{GRAVITY, IDLE_FRAME_COUNT, IDLE_FRAME_DURATION, JUMP_FRAME_COUNT, JUMP_FRAME_DURATION, JUMP_POWER, PLAYER_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH, WALK_FRAME_COUNT, WALK_FRAME_DURATION};
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use macroquad::math::Vec2;
use macroquad::prelude::get_frame_time;
use macroquad::texture::Texture2D;

#[derive(PartialEq)]
enum CharacterState {
    Idle,
    Walk,
}

pub struct Character {
    pub is_jumping: bool,
    pub on_ground: bool,
    pub pos_x: f32,
    pub pos_y: f32,
    pub animate_idle: Animate,
    pub animate_walk: Animate,
    pub animate_jump: Animate,
    pub jump_vec: Vec2,
    direction: CharacterDirection,
    state: CharacterState,
}

impl Character {
    pub fn new(idle_texture: &Texture2D, walk_texture: &Texture2D, jump_texture: &Texture2D) -> Self {
        Self {
            pos_x: 0.0,
            pos_y: SCREEN_HEIGHT - idle_texture.height() - 30.0,
            direction: CharacterDirection::Right,
            state: CharacterState::Idle,
            animate_idle: Animate::new(&idle_texture, IDLE_FRAME_COUNT, IDLE_FRAME_DURATION),
            animate_walk: Animate::new(&walk_texture, WALK_FRAME_COUNT, WALK_FRAME_DURATION),
            animate_jump: Animate::new(&jump_texture, JUMP_FRAME_COUNT, JUMP_FRAME_DURATION),
            is_jumping: false,
            on_ground: false,
            jump_vec: Vec2::new(0.0, 0.0),
        }
    }

    pub fn handle_keys(&mut self) {
        let mut new_pos_x = self.pos_x;
        let dt = get_frame_time();

        if is_key_down(KeyCode::Left) {
            new_pos_x -= PLAYER_SPEED * dt;
            self.direction = CharacterDirection::Left;
            self.state = CharacterState::Walk;
        } else if is_key_down(KeyCode::Right) {
            new_pos_x += PLAYER_SPEED * dt;
            self.direction = CharacterDirection::Right;
            self.state = CharacterState::Walk;
        } else {
            self.state = CharacterState::Idle;
        }

        if is_key_pressed(KeyCode::Space) && self.on_ground {
            self.is_jumping = true;
            self.on_ground = false;
            self.jump_vec.y = -JUMP_POWER;
        }

        self.jump_vec.y += GRAVITY * dt; // v = v0 + a⋅t

        let mut new_pos_y = self.pos_y + self.jump_vec.y * dt; // y = y0 + v⋅t

        let current_frame_width = match self.state {
            CharacterState::Idle => self.animate_idle.frame_size.x,
            CharacterState::Walk => self.animate_walk.frame_size.x,
        };

        let current_frame_height = match self.state {
            CharacterState::Idle => self.animate_idle.frame_size.y,
            CharacterState::Walk => self.animate_walk.frame_size.y,
        };

        if new_pos_x >= 0.0 && new_pos_x + current_frame_width <= SCREEN_WIDTH {
            self.pos_x = new_pos_x;
        }

        if new_pos_y + current_frame_height >= SCREEN_HEIGHT - 30.0 {
            new_pos_y = SCREEN_HEIGHT - current_frame_height - 30.0 ;
            self.jump_vec.y = 0.0;
            self.is_jumping = false;
            self.on_ground = true;
        }

        self.pos_y = new_pos_y;
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        if self.state == CharacterState::Walk {
            self.animate_walk.update(dt);
        } else {
            self.animate_idle.update(dt);
        }

        if self.is_jumping {
            self.animate_jump.update(dt);
        }
    }

    pub fn draw(&mut self) {
        if self.is_jumping {
            self.animate_jump
                .draw(Vec2::new(self.pos_x, self.pos_y), &self.direction);
        } else if self.state == CharacterState::Walk {
            self.animate_walk
                .draw(Vec2::new(self.pos_x, self.pos_y), &self.direction);
        } else {
            self.animate_idle
                .draw(Vec2::new(self.pos_x, self.pos_y), &self.direction);
        }
    }
}
