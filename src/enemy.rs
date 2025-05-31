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
    hit_timer: f32,  // Manual timer for hit animation
    die_timer: f32,  // Manual timer for die animation
    hit_duration: f32,  // Total duration of hit animation
    die_duration: f32,  // Total duration of die animation
}

impl Enemy {
    pub fn new(
        animate_texture: &Texture2D,
        enemy_fly_state: &Texture2D,
        enemy_hit_state: &Texture2D,
        enemy_die_state: &Texture2D,
    ) -> Self {
        let pos_y = animate_texture.height() * 2.0;

        Self {
            animate_idle: Animate::new(animate_texture, 8, 0.125),
            animate_fly: Animate::new(enemy_fly_state, 8, 0.125),
            animate_hit: Animate::new(enemy_hit_state, 4, 0.15),
            animate_die: Animate::new(enemy_die_state, 16, 0.08),
            velocity: Vec2::new(150.0, 0.0),
            rect: Rect::new(
                10.0,
                pos_y,
                animate_texture.height() - 10.0,
                animate_texture.height(),
            ),
            direction: CharacterDirection::Right,
            speed: 150.0,
            lives: 0,
            state: EnemyState::Fly,
            stored_velocity: Vec2::new(150.0, 0.0),
            hit_timer: 0.0,
            die_timer: 0.0,
            hit_duration: 4.0 * 0.15,  // 4 frames * 0.15 seconds per frame
            die_duration: 16.0 * 0.08, // 15 frames * 0.08 seconds per frame
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        match self.state {
            EnemyState::Fly => {
                // Normal flying behavior
                if self.velocity.x > 0.0 {
                    self.direction = CharacterDirection::Right;
                } else if self.velocity.x < 0.0 {
                    self.direction = CharacterDirection::Left;
                }

                self.rect.x += self.velocity.x * dt;

                // Bounce off edges
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
                // Update hit timer
                self.hit_timer += dt;

                // Update hit animation normally (it will loop, but we control when to stop)
                self.animate_hit.update(dt);

                // Check if hit animation duration is complete
                if self.hit_timer >= self.hit_duration {
                    self.state = EnemyState::Fly;
                    self.velocity = self.stored_velocity; // Restore movement
                    self.hit_timer = 0.0; // Reset timer
                    // Reset hit animation to first frame for next time
                    self.animate_hit.reset();
                }
            },

            EnemyState::Die => {
                // Update die timer
                self.die_timer += dt;

                // Only update animation if not finished
                if self.die_timer < self.die_duration {
                    self.animate_die.update(dt);
                }
                // Animation stops on last frame when timer expires
            },
        }
    }

    pub fn hit(&mut self) {
        if matches!(self.state, EnemyState::Die) {
            return;
        }

        if self.lives == 0 {
            // Start die animation
            self.state = EnemyState::Die;
            self.stored_velocity = Vec2::new(0.0, 0.0);
            self.die_timer = 0.0;
            self.animate_die.reset();
        } else {
            // Take damage and play hit animation
            self.lives = self.lives.saturating_sub(1);
            self.state = EnemyState::Hit;
            self.stored_velocity = self.velocity;
            self.hit_timer = 0.0;
            self.animate_hit.reset();
        }
    }

    pub fn draw(&mut self) {
        let draw_pos = Vec2::new(self.rect.x, self.rect.y);

        match self.state {
            EnemyState::Fly => {
                self.animate_fly.draw(draw_pos, &self.direction);
            },
            EnemyState::Hit => {
                self.animate_hit.draw(draw_pos, &self.direction);
            },
            EnemyState::Die => {
                self.animate_die.draw(draw_pos, &self.direction);
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

    // Helper methods for state checking
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
}