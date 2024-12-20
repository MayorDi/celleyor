use std::ptr::null;

use constants::{SIZE_GRID, SIZE_RENDER_CELL_GRID};
use layout::Layout;

use crate::{
    cell::Cell,
    control::Camera,
    opengl::prelude::{get_location, GetId, Program, Shader, Vao, Vbo},
    zone::Zone,
};

pub mod constants;
pub mod layout;

pub struct Grid {
    pub layout_zones: Layout<Zone>,
    pub layout_cells: Layout<Cell>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            layout_zones: Layout::new(),
            layout_cells: Layout::new(),
        }
    }

    pub fn create_render_info(&self) -> (Vao, Vbo) {
        let mut vao @ mut vbo = 0;
        let vertices = [
            0.0,
            0.0,
            0.0 + SIZE_GRID[0] as f32 * SIZE_RENDER_CELL_GRID,
            0.0,
            0.0 + SIZE_GRID[0] as f32 * SIZE_RENDER_CELL_GRID,
            0.0 + SIZE_GRID[1] as f32 * SIZE_RENDER_CELL_GRID,
            0.0,
            0.0 + SIZE_GRID[1] as f32 * SIZE_RENDER_CELL_GRID,
        ];

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * size_of::<f32>()) as isize,
                    vertices.as_ptr() as _,
                    gl::STATIC_DRAW,
                );

                gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, null() as _);
                gl::EnableVertexAttribArray(0);
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        (Vao(vao), Vbo(vbo))
    }

    pub fn render_grid(
        &self,
        camera: &Camera,
        resolution: (f32, f32),
        program: &Program<Shader>,
        vao: Vao,
    ) {
        unsafe {
            gl::BindVertexArray(vao.0);
            {
                gl::UseProgram(program.id());
                gl::Uniform2f(
                    get_location(program, "u_resolution"),
                    resolution.0,
                    resolution.1,
                );
                gl::Uniform2f(
                    get_location(program, "u_camera_pos"),
                    camera.position.x,
                    camera.position.y,
                );
                gl::Uniform1f(get_location(program, "u_camera_scale"), camera.scale);
                gl::DrawArrays(gl::LINE_LOOP, 0, 4);
                gl::UseProgram(0);
            }
            gl::BindVertexArray(0);
        }
    }
}
