use crate::animate::{Animate, CharacterDirection};
use crate::constants::SCREEN_WIDTH;
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{Texture2D, get_frame_time};

pub enum EnemyState {
    Fly,
    Hit,
    Die,
}

pub struct Enemy {
    pub animate_idle: Animate,
    pub animate_fly: Animate,
    pub animate_hit: Animate,
    pub animate_die: Animate,
    pub velocity: Vec2,
    pub rect: Rect,
    direction: CharacterDirection,
    pub speed: f32,
    pub lives: u8,
    pub state: EnemyState,
    stored_velocity: Vec2,
    hit_timer: f32,
    die_timer: f32,
    hit_duration: f32,
    die_duration: f32,
}

impl Enemy {
    pub fn new(
        animate_texture: &Vec<Texture2D>,
        enemy_fly_state: &Vec<Texture2D>,
        enemy_hit_state: &Vec<Texture2D>,
        enemy_die_state: &Vec<Texture2D>,
    ) -> Self {
        let pos_y = animate_texture[0].height() * 2.0;

        Self {
            animate_idle: Animate::new_normalized(animate_texture, 0.125),
            animate_fly: Animate::new_normalized(enemy_fly_state, 0.125),
            animate_hit: Animate::new_normalized(enemy_hit_state, 0.15),
            animate_die: Animate::new_normalized(enemy_die_state, 0.08),
            velocity: Vec2::new(150.0, 0.0),
            rect: Rect::new(
                10.0,
                pos_y,
                animate_texture[0].height() - 10.0,
                animate_texture[0].height(),
            ),
            direction: CharacterDirection::Right,
            speed: 150.0,
            lives: 1,
            state: EnemyState::Fly,
            stored_velocity: Vec2::new(150.0, 0.0),
            hit_timer: 0.0,
            die_timer: 0.0,
            hit_duration: 4.0 * 0.15,
            die_duration: 16.0 * 0.08,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        match self.state {
            EnemyState::Fly => {
                if self.velocity.x > 0.0 {
                    self.direction = CharacterDirection::Right;
                } else if self.velocity.x < 0.0 {
                    self.direction = CharacterDirection::Left;
                }

                self.rect.x += self.velocity.x * dt;

                if self.rect.x <= 0.0 {
                    self.rect.x = 0.0;
                    self.velocity.x = self.speed;
                    self.direction = CharacterDirection::Right;
                } else if self.rect.x + self.rect.w >= SCREEN_WIDTH {
                    self.rect.x = SCREEN_WIDTH - self.rect.w;
                    self.velocity.x = -self.speed;
                    self.direction = CharacterDirection::Left;
                }

                self.animate_fly.update(dt);
            },

            EnemyState::Hit => {
                self.hit_timer += dt;

                self.animate_hit.update(dt);

                if self.hit_timer >= self.hit_duration {
                    self.state = EnemyState::Fly;
                    self.velocity = self.stored_velocity;
                    self.hit_timer = 0.0;
                    self.animate_hit.reset();
                }
            },

            EnemyState::Die => {
                self.die_timer += dt;

                if self.die_timer < self.die_duration {
                    self.animate_die.update(dt);
                }
            },
        }
    }

    pub fn hit(&mut self) {
        if matches!(self.state, EnemyState::Die) {
            return;
        }

        if self.lives == 0 {
            self.state = EnemyState::Die;
            self.velocity = Vec2::new(0.0, 0.0);
            self.stored_velocity = Vec2::new(0.0, 0.0);
            self.die_timer = 0.0;
            self.animate_die.reset();
        } else {
            self.lives = self.lives.saturating_sub(1);
            self.state = EnemyState::Hit;
            self.stored_velocity = self.velocity;
            self.velocity = Vec2::new(0.0, 0.0);
            self.hit_timer = 0.0;
            self.animate_hit.reset();
        }
    }

    pub fn draw(&mut self) {
        let ground_pos = Vec2::new(self.rect.x, self.rect.y + self.rect.h);

        match self.state {
            EnemyState::Fly => {
                self.animate_fly.draw_bottom_aligned(ground_pos, &self.direction);
            },
            EnemyState::Hit => {
                self.animate_hit.draw_bottom_aligned(ground_pos, &self.direction);
            },
            EnemyState::Die => {
                self.animate_die.draw_bottom_aligned(ground_pos, &self.direction);
            },
        }
    }

    pub fn reset_position(&mut self) {
        self.rect.x = 10.0;
        self.velocity.x = self.speed;
        self.direction = CharacterDirection::Right;
        self.state = EnemyState::Fly;
        self.lives = 3;
        self.hit_timer = 0.0;
        self.die_timer = 0.0;
    }

    pub fn reverse_direction(&mut self) {
        self.velocity.x = -self.velocity.x;
        self.direction = if self.velocity.x > 0.0 {
            CharacterDirection::Right
        } else {
            CharacterDirection::Left
        };
        self.stored_velocity = self.velocity;
    }

    pub fn is_at_edge(&self) -> bool {
        self.rect.x <= 0.0 || self.rect.x + self.rect.w >= SCREEN_WIDTH
    }

    pub fn is_dead(&self) -> bool {
        matches!(self.state, EnemyState::Die) && self.die_timer >= self.die_duration
    }

    pub fn is_dying(&self) -> bool {
        matches!(self.state, EnemyState::Die)
    }

    pub fn is_hit(&self) -> bool {
        matches!(self.state, EnemyState::Hit)
    }

    pub fn can_be_hit(&self) -> bool {
        matches!(self.state, EnemyState::Fly)
    }

    pub fn enable_debug(&mut self) {
        self.animate_idle.enable_debug();
        self.animate_fly.enable_debug();
        self.animate_hit.enable_debug();
        self.animate_die.enable_debug();
    }

    pub fn disable_debug(&mut self) {
        self.animate_idle.disable_debug();
        self.animate_fly.disable_debug();
        self.animate_hit.disable_debug();
        self.animate_die.disable_debug();
    }

    pub fn toggle_debug(&mut self) {
        self.animate_idle.toggle_debug();
        self.animate_fly.toggle_debug();
        self.animate_hit.toggle_debug();
        self.animate_die.toggle_debug();
    }

    pub fn set_debug(&mut self, enabled: bool) {
        self.animate_idle.set_debug(enabled);
        self.animate_fly.set_debug(enabled);
        self.animate_hit.set_debug(enabled);
        self.animate_die.set_debug(enabled);
    }

    pub fn get_current_animation_info(&self) -> (u8, u8, Vec2) {
        match self.state {
            EnemyState::Fly => (
                self.animate_fly.get_current_frame(),
                self.animate_fly.get_total_frames(),
                self.animate_fly.get_max_frame_size()
            ),
            EnemyState::Hit => (
                self.animate_hit.get_current_frame(),
                self.animate_hit.get_total_frames(),
                self.animate_hit.get_max_frame_size()
            ),
            EnemyState::Die => (
                self.animate_die.get_current_frame(),
                self.animate_die.get_total_frames(),
                self.animate_die.get_max_frame_size()
            ),
        }
    }

    pub fn draw_with_custom_positioning(&mut self) {
        let draw_pos = Vec2::new(self.rect.x, self.rect.y);
        let ground_pos = Vec2::new(self.rect.x, self.rect.y + self.rect.h);

        match self.state {
            EnemyState::Fly => {
                self.animate_fly.draw_bottom_aligned(ground_pos, &self.direction);
            },
            EnemyState::Hit => {
                self.animate_hit.draw_centered(draw_pos, &self.direction);
            },
            EnemyState::Die => {
                self.animate_die.draw(draw_pos, &self.direction);
            },
        }
    }

    pub fn get_current_max_frame_size(&self) -> Vec2 {
        match self.state {
            EnemyState::Fly => self.animate_fly.get_max_frame_size(),
            EnemyState::Hit => self.animate_hit.get_max_frame_size(),
            EnemyState::Die => self.animate_die.get_max_frame_size(),
        }
    }

    pub fn update_collision_rect_to_animation(&mut self) {
        let max_size = self.get_current_max_frame_size();
        let current_x = self.rect.x;
        let current_y = self.rect.y;

        self.rect.w = max_size.x;
        self.rect.h = max_size.y;

        self.rect.x = current_x;
        self.rect.y = current_y;
    }
}