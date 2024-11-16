use glfw::MouseButton;
use nalgebra::Vector2;

#[derive(Debug)]
pub struct Mouse {
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
    pub button: MouseButton,
    pub pressed: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            position: Default::default(),
            old_position: Default::default(),
            button: MouseButton::Button1,
            pressed: false,
        }
    }

    pub fn delta(&self) -> Vector2<f32> {
        self.position - self.old_position
    }
}
