use macroquad::color::WHITE;
use macroquad::math::{vec2, Rect, Vec2};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};

pub struct Arrow {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub rect: Rect,
    texture: Texture2D,
    pub stuck: bool,
    pub stuck_angle: f32,
    pub stuck_timer: f32,
    pub should_remove: bool,
}

impl Arrow {
    pub fn new(pos: Vec2, velocity: Vec2, texture: Texture2D) -> Self {
        let size = vec2(texture.width(), texture.height());
        let rect = Rect::new(pos.x, pos.y, size.x, size.y);

        Arrow {
            pos,
            velocity,
            rect,
            texture,
            stuck: false,
            stuck_angle: 0.0,
            stuck_timer: 0.0,
            should_remove: false,
        }
    }

    pub fn get_tip_collision_point(&self) -> Vec2 {
        let angle = if self.stuck {
            self.stuck_angle
        } else {
            self.velocity.y.atan2(self.velocity.x)
        };

        let tip_offset = self.rect.w * 0.8;
        let tip_x = self.pos.x + self.rect.w/2.0 + angle.cos() * tip_offset;
        let tip_y = self.pos.y + self.rect.h/2.0 + angle.sin() * tip_offset;

        vec2(tip_x, tip_y)
    }

    pub fn check_collision_and_embed(&mut self, tile_rect: &Rect, embed_depth: f32) -> bool {
        let tip = self.get_tip_collision_point();

        if tile_rect.contains(tip) {
            let angle = self.velocity.y.atan2(self.velocity.x);

            self.pos.x += angle.cos() * embed_depth;
            self.pos.y += angle.sin() * embed_depth;

            self.rect.x = self.pos.x;
            self.rect.y = self.pos.y;

            return true;
        }

        false
    }

    pub fn update(&mut self, dt: f32) {
        if self.stuck {
            self.stuck_timer += dt;
            if self.stuck_timer >= 3.0 {
                self.should_remove = true;
            }
            return;
        }

        let gravity = vec2(0.0, 200.0);
        self.velocity += gravity * dt;
        self.pos += self.velocity * dt;

        self.rect.x = self.pos.x;
        self.rect.y = self.pos.y;
    }

    pub fn draw(&self) {
        let angle = if self.stuck {
            self.stuck_angle
        } else {
            self.velocity.y.atan2(self.velocity.x)
        };

        draw_texture_ex(
            &self.texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                rotation: angle,
                dest_size: Some(vec2(self.rect.w, self.rect.h)),
                ..Default::default()
            },
        );
    }
}