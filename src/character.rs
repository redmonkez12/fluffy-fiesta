use crate::animate::{Animate, CharacterDirection};
use crate::arrow::Arrow;
use crate::constants::{
    ATTACK_FRAME_DURATION, BOW_FRAME_DURATION, GRAVITY,
    IDLE_FRAME_DURATION, JUMP_FRAME_DURATION, JUMP_POWER,
    PLAYER_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH, WALK_FRAME_DURATION,
};
use macroquad::input::{is_key_down, is_key_pressed, mouse_position, KeyCode, MouseButton};
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{get_frame_time, is_mouse_button_pressed, Texture2D};

#[derive(PartialEq)]
enum CharacterState {
    Idle,
    Walk,
    Jump,
    Attack,
}

pub struct Character {
    pub is_jumping: bool,
    pub on_ground: bool,
    pub animate_idle: Animate,
    // pub animate_walk: Animate,
    // pub animate_jump: Animate,
    // pub animate_attack: Animate,
    // pub animate_bow: Animate,
    pub velocity: Vec2,
    pub rect: Rect,
    pub arrows: Vec<Arrow>,
    // pub arrow_texture: Texture2D,
    direction: CharacterDirection,
    state: CharacterState,
    is_shooting: bool,
    shoot_timer: f32,
    shoot_duration: f32,
    bow_angle: f32,
    show_bow: bool,
}

impl Character {
    pub fn new(
        idle_textures: &Vec<Texture2D>,    // Changed: now Vec<Texture2D>
    ) -> Self {
        let pos_y = SCREEN_HEIGHT - idle_textures[0].height() * 3.0;

        Self {
            direction: CharacterDirection::Right,
            state: CharacterState::Idle,
            animate_idle: Animate::new(idle_textures, IDLE_FRAME_DURATION),
            // animate_walk: Animate::new(walk_textures, WALK_FRAME_DURATION),
            // animate_jump: Animate::new(jump_textures, JUMP_FRAME_DURATION),
            // animate_attack: Animate::new(attack_textures, ATTACK_FRAME_DURATION),
            // animate_bow: Animate::new(bow_textures, BOW_FRAME_DURATION),
            is_jumping: false,
            on_ground: true,
            velocity: Vec2::new(0.0, 0.0),
            rect: Rect::new(
                52.0,
                pos_y,
                idle_textures[0].width() - 10.0,
                idle_textures[0].height(),
            ),
            arrows: Vec::new(),
            // arrow_texture: arrow_texture.clone(),
            is_shooting: false,
            shoot_timer: 0.0,
            shoot_duration: 0.2,
            bow_angle: 0.0,
            show_bow: false,
        }
    }

    pub fn handle_keys(&mut self) {
        let dt = get_frame_time();

        if self.is_shooting {
            self.shoot_timer += dt;
            if self.shoot_timer >= self.shoot_duration {
                self.is_shooting = false;
                self.shoot_timer = 0.0;
                self.show_bow = false;
                self.state = CharacterState::Idle;
            }
        }

        if is_key_down(KeyCode::A) {
            self.velocity.x = -PLAYER_SPEED;
            self.direction = CharacterDirection::Left;
            self.state = CharacterState::Walk;
        } else if is_key_down(KeyCode::D) {
            self.velocity.x = PLAYER_SPEED;
            self.direction = CharacterDirection::Right;
            self.state = CharacterState::Walk;
        } else {
            if !self.is_shooting {
                self.state = CharacterState::Idle;
            }
            self.velocity.x = 0.0;
        }

        if is_key_pressed(KeyCode::W) && self.on_ground {
            self.state = CharacterState::Jump;
            self.is_jumping = true;
            self.on_ground = false;
            self.velocity.y = -JUMP_POWER;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.shoot();
            self.state = CharacterState::Attack;
        }

        self.velocity.y += GRAVITY * dt;

        self.rect.x = self.rect.x + self.velocity.x * dt;
        self.rect.y = self.rect.y + self.velocity.y * dt;

        // Ground collision
        if self.rect.y + self.rect.h >= SCREEN_HEIGHT {
            self.rect.y = SCREEN_HEIGHT - self.rect.h;
            self.velocity.y = 0.0;
            self.is_jumping = false;
            self.on_ground = true;
            if self.state == CharacterState::Jump {
                self.state = CharacterState::Idle;
            }
        }

        // Screen boundaries
        if self.rect.x < 0.0 {
            self.rect.x = 0.0;
        } else if self.rect.x + self.rect.w > SCREEN_WIDTH {
            self.rect.x = SCREEN_WIDTH - self.rect.w;
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        match self.state {
            CharacterState::Walk => self.animate_idle.update(dt),
            CharacterState::Jump => self.animate_idle.update(dt),
            CharacterState::Attack => self.animate_idle.update(dt),
            CharacterState::Idle => self.animate_idle.update(dt),
        }

        if self.is_shooting && self.show_bow {
            self.animate_idle.update(dt);
        }

        // Update arrows
        // for arrow in &mut self.arrows {
        //     arrow.update();
        // }

        // Remove arrows that are off-screen
        // self.arrows.retain(|arrow| {
        //     arrow.position.x >= -50.0 && arrow.position.x <= SCREEN_WIDTH + 50.0 &&
        //         arrow.position.y >= -50.0 && arrow.position.y <= SCREEN_HEIGHT + 50.0
        // });
    }

    pub fn draw(&mut self) {
        let draw_pos = Vec2::new(self.rect.x, self.rect.y);

        match self.state {
            CharacterState::Walk => self.animate_idle.draw(draw_pos, &self.direction),
            CharacterState::Jump => self.animate_idle.draw(draw_pos, &self.direction),
            CharacterState::Attack => self.animate_idle.draw(draw_pos, &self.direction),
            CharacterState::Idle => self.animate_idle.draw(draw_pos, &self.direction),
        }

        if self.show_bow {
            self.draw_bow();
        }

        // Draw arrows
        for arrow in &mut self.arrows {
            arrow.draw();
        }
    }

    fn draw_bow(&mut self) {
        let bow_offset = match self.direction {
            CharacterDirection::Right => Vec2::new(0.0, 4.0),
            CharacterDirection::Left => Vec2::new(10.0, 0.0),
        };

        let character_pos = Vec2::new(self.rect.center().x, self.rect.center().y);
        let bow_pos = character_pos + bow_offset;

        // self.animate_bow.draw_rotated(bow_pos, &self.direction, self.bow_angle);
    }

    fn shoot(&mut self) {
        let mouse = Vec2::from(mouse_position());

        let bow_offset = match self.direction {
            CharacterDirection::Right => Vec2::new(-5.0, 4.0),
            CharacterDirection::Left => Vec2::new(0.0, 1.0),
        };

        let mut bow_pos = self.get_bow_position();
        let arrow_start = bow_pos + bow_offset;

        if self.direction == CharacterDirection::Left {
            bow_pos.x += 100.0;
        }

        let direction = (mouse - arrow_start).normalize();
        let angle = direction.y.atan2(direction.x);

        let can_shoot = match self.direction {
            CharacterDirection::Right => {
                angle >= -std::f32::consts::FRAC_PI_2 && angle <= std::f32::consts::FRAC_PI_2
            }
            CharacterDirection::Left => {
                angle >= std::f32::consts::FRAC_PI_2 || angle <= -std::f32::consts::FRAC_PI_2
            }
        };

        if !can_shoot {
            return;
        }

        self.is_shooting = true;
        self.show_bow = true;
        self.shoot_timer = 0.0;
        self.bow_angle = angle;

        let speed = 400.0;
        let velocity = direction * speed;
        // let arrow = Arrow::new(arrow_start, velocity, self.arrow_texture.clone());
        // self.arrows.push(arrow);
    }

    pub fn set_bow_visibility(&mut self, visible: bool) {
        self.show_bow = visible;
    }

    pub fn toggle_bow(&mut self) {
        self.show_bow = !self.show_bow;
    }

    pub fn set_shoot_duration(&mut self, duration: f32) {
        self.shoot_duration = duration;
    }

    fn get_bow_position(&self) -> Vec2 {
        Vec2::new(self.rect.center().x, self.rect.center().y)
    }
}