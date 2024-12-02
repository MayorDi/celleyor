use nalgebra::Vector2;

use super::Mouse;

#[derive(Debug, Default)]
pub struct Camera {
    pub position: Vector2<f32>,
    pub scale: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Default::default(),
            scale: 1.0,
        }
    }

    pub fn mouse_move(&mut self, mouse: &Mouse) {
        self.position += mouse.delta() / self.scale;
    }

    pub fn update_scale(&mut self, new_scale: f64) {
        if (self.scale + new_scale as f32) > 0.0 {
            self.scale += new_scale as f32 / 2.0;
        }
    }
}
