use macroquad::prelude::*;

#[derive(Debug)]
pub struct WorldCamera {
    pub position: Vec2,
    pub target_pos: Vec2,
    pub smoothing: f32,
    pub bounds: Rect,
}

impl WorldCamera {
    pub fn new(map_width: f32, map_height: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            target_pos: Vec2::ZERO,
            smoothing: 5.0,
            bounds: Rect::new(0.0, 0.0, map_width, map_height),
        }
    }

    pub fn follow_target(&mut self, target_pos: Vec2, dt: f32) {
        self.target_pos = target_pos;

        let desired_pos = Vec2::new(
            target_pos.x - screen_width() * 0.5,
            target_pos.y - screen_height() * 0.5,
        );

        let max_x = (self.bounds.w - screen_width()).max(0.0);

        let min_y = f32::NEG_INFINITY;
        let max_y = (self.bounds.h - screen_height()).min(0.0);

        let clamped_pos = Vec2::new(
            desired_pos.x.clamp(0.0, max_x),
            desired_pos.y.clamp(min_y, max_y),
        );

        self.position = self.position.lerp(clamped_pos, self.smoothing * dt);
    }

    pub fn get_camera2d(&self) -> Camera2D {
        println!("{:?}", self.position);
        Camera2D {
            target: self.position + vec2(screen_width() * 0.5, screen_height() * 0.5),
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            offset: vec2(0.0, 0.0),
            rotation: 0.0,
            render_target: None,
            viewport: None,
        }
    }
}
