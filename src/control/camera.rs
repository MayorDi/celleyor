use nalgebra::Vector2;

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
}