use std::ptr::null;

use constants::{SIZE_GRID, SIZE_RENDER_CELL_GRID};

use crate::{
    cell::Cell,
    control::Camera,
    opengl::prelude::{get_location, Build, GetId, Program, Shader, Vao, Vbo},
};

pub mod constants;

pub struct Grid {
    layout_cells: [[Option<Cell>; SIZE_GRID[0]]; SIZE_GRID[1]],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            layout_cells: [const { [const { None }; SIZE_GRID[0]] }; SIZE_GRID[1]],
        }
    }

    pub fn build_render_program() -> Program<Shader> {
        let vs = Shader::new(
            gl::VERTEX_SHADER,
            include_bytes!("../../res/shaders/grid/grid.vert").to_vec(),
        );
        let fs = Shader::new(
            gl::FRAGMENT_SHADER,
            include_bytes!("../../res/shaders/grid/grid.frag").to_vec(),
        );

        let mut program = Program::new();
        program.push_shader(vs);
        program.push_shader(fs);
        program.build().unwrap();

        program
    }

    pub fn init_render_grid(&self) -> (Vao, Vbo) {
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
