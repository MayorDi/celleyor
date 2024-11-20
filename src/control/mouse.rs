use glfw::{Action, MouseButton};
use nalgebra::Vector2;

use crate::{
    game::tools::{SelectTools, Tools},
    grid::Grid,
};

use super::Camera;

#[derive(Debug)]
pub struct Mouse {
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
    pub world_position: Vector2<f32>,
    pub grid_position: Vector2<usize>,
    pub button: MouseButton,
    pub pressed: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            position: Default::default(),
            old_position: Default::default(),
            world_position: Default::default(),
            grid_position: Default::default(),
            button: MouseButton::Button1,
            pressed: false,
        }
    }

    pub fn update_world_position(&mut self, camera: &Camera, resolution: (f32, f32)) {
        self.world_position = nalgebra::Vector2::new(
            camera.position.x / 2.0 + (self.position.x - resolution.0 / 2.0) / camera.scale,
            camera.position.y / 2.0 + (resolution.1 / 2.0 - self.position.y) / camera.scale,
        );
    }

    pub fn update_grid_position(&mut self) {
        self.grid_position = nalgebra::Vector2::new(
            (self.world_position.x / 8.0) as usize,
            (self.world_position.y / 8.0) as usize,
        );
    }

    pub fn delta(&self) -> Vector2<f32> {
        Vector2::new(
            self.old_position.x - self.position.x,
            self.position.y - self.old_position.y,
        )
    }

    pub fn event_tools(&self, tools: &Tools, grid: &mut Grid) {
        match tools.select_tools {
            SelectTools::AddNewZone => grid.layout_zones[self.grid_position] = Some(tools.zone),
            SelectTools::None => {}
        }
    }

    pub fn event_button(&mut self, button: &MouseButton, action: &Action) {
        self.button = *button;

        match action {
            glfw::Action::Press => self.pressed = true,
            _ => self.pressed = false,
        }
    }

    pub fn event_action(&self, camera: &mut Camera, tools: &Tools, grid: &mut Grid) {
        if !self.pressed {
            return;
        }

        match self.button {
            glfw::MouseButton::Button3 => camera.mouse_move(self),
            glfw::MouseButton::Button2 => self.event_tools(&tools, grid),
            _ => {}
        }
    }
}
