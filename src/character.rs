use crate::animate::{Animate, CharacterDirection};
use crate::constants::{
    GRAVITY, IDLE_FRAME_COUNT, IDLE_FRAME_DURATION, JUMP_FRAME_COUNT, JUMP_FRAME_DURATION,
    JUMP_POWER, PLAYER_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH, WALK_FRAME_COUNT, WALK_FRAME_DURATION,
};
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{get_frame_time, Texture2D};

#[derive(PartialEq)]
enum CharacterState {
    Idle,
    Walk,
    Jump,
}

pub struct Character {
    pub is_jumping: bool,
    pub on_ground: bool,
    pub animate_idle: Animate,
    pub animate_walk: Animate,
    pub animate_jump: Animate,
    pub velocity: Vec2,
    pub rect: Rect,
    direction: CharacterDirection,
    state: CharacterState,
}

impl Character {
    pub fn new(idle_texture: &Texture2D, walk_texture: &Texture2D, jump_texture: &Texture2D) -> Self {
        let pos_y = SCREEN_HEIGHT - idle_texture.height() * 3.0;

        Self {
            direction: CharacterDirection::Right,
            state: CharacterState::Idle,
            animate_idle: Animate::new(idle_texture, IDLE_FRAME_COUNT, IDLE_FRAME_DURATION),
            animate_walk: Animate::new(walk_texture, WALK_FRAME_COUNT, WALK_FRAME_DURATION),
            animate_jump: Animate::new(jump_texture, JUMP_FRAME_COUNT, JUMP_FRAME_DURATION),
            is_jumping: false,
            on_ground: true,
            velocity: Vec2::new(0.0, 0.0),
            rect: Rect::new(10.0, pos_y, idle_texture.height(), idle_texture.height()),
        }
    }

    pub fn handle_keys(&mut self) {
        let dt = get_frame_time();

        if is_key_down(KeyCode::Left) {
            self.velocity.x = -PLAYER_SPEED;
            self.direction = CharacterDirection::Left;
            self.state = CharacterState::Walk;
        } else if is_key_down(KeyCode::Right) {
            self.velocity.x = PLAYER_SPEED;
            self.direction = CharacterDirection::Right;
            self.state = CharacterState::Walk;
        } else {
            self.state = CharacterState::Idle;
            self.velocity.x = 0.0;
        }

        if is_key_pressed(KeyCode::Space) && self.on_ground {
            self.state = CharacterState::Jump;
            self.is_jumping = true;
            self.on_ground = false;
            self.velocity.y = -JUMP_POWER;
        }

        self.velocity.y += GRAVITY * dt;

        let mut new_pos_x = self.rect.x + self.velocity.x * dt;
        let mut new_pos_y = self.rect.y + self.velocity.y * dt;

        let current_frame_width = match self.state {
            CharacterState::Idle => self.animate_idle.frame_size.x,
            CharacterState::Walk => self.animate_walk.frame_size.x,
            CharacterState::Jump => self.animate_jump.frame_size.x,
        };

        let current_frame_height = match self.state {
            CharacterState::Idle => self.animate_idle.frame_size.y,
            CharacterState::Walk => self.animate_walk.frame_size.y,
            CharacterState::Jump => self.animate_jump.frame_size.y,
        };

        if new_pos_x >= 0.0 && new_pos_x + current_frame_width <= SCREEN_WIDTH {
            self.rect.x = new_pos_x;
        }

        if new_pos_y + current_frame_height >= SCREEN_HEIGHT {
            new_pos_y = SCREEN_HEIGHT - current_frame_height;
            self.velocity.y = 0.0;
            self.is_jumping = false;
            self.state = CharacterState::Idle;
            self.on_ground = true;
        }

        self.rect.y = new_pos_y;
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
                .draw(Vec2::new(self.rect.x, self.rect.y), &self.direction);
        } else if self.state == CharacterState::Walk {
            self.animate_walk
                .draw(Vec2::new(self.rect.x, self.rect.y), &self.direction);
        } else {
            self.animate_idle
                .draw(Vec2::new(self.rect.x, self.rect.y), &self.direction);
        }
    }
}
