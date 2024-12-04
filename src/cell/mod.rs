use nalgebra::Vector2;

use crate::{
    control::Camera,
    grid::{constants::SIZE_RENDER_CELL_GRID, layout::Layout},
    opengl::prelude::{get_location, GetId, Program, Shader, Vao, Vbo}, uniform,
};

#[derive(Debug, Clone)]
pub struct Cell {
    mass: f32,
    energy: f32,
    pub color: [f32; 3],
}

impl Cell {
    pub fn create_render_data(&self, pos: Vector2<usize>) -> [f32; 42] {
        let (x, y) = (
            pos.x as f32 * SIZE_RENDER_CELL_GRID,
            pos.y as f32 * SIZE_RENDER_CELL_GRID,
        );
        let vertices = [
            x,
            y,
            0.0,
            0.0,
            self.color[0],
            self.color[1],
            self.color[2],
            x + SIZE_RENDER_CELL_GRID,
            y,
            1.0,
            0.0,
            self.color[0],
            self.color[1],
            self.color[2],
            x,
            y + SIZE_RENDER_CELL_GRID,
            0.0,
            1.0,
            self.color[0],
            self.color[1],
            self.color[2],
            x,
            y + SIZE_RENDER_CELL_GRID,
            0.0,
            1.0,
            self.color[0],
            self.color[1],
            self.color[2],
            x + SIZE_RENDER_CELL_GRID,
            y + SIZE_RENDER_CELL_GRID,
            1.0,
            1.0,
            self.color[0],
            self.color[1],
            self.color[2],
            x + SIZE_RENDER_CELL_GRID,
            y,
            1.0,
            0.0,
            self.color[0],
            self.color[1],
            self.color[2],
        ];

        vertices
    }

    pub fn create_render_info() -> (Vao, Vbo) {
        let mut vao @ mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
        }

        (Vao(vao), Vbo(vbo))
    }

    pub fn init_render_cells(layout_cells: &Layout<Cell>, vao: Vao, vbo: Vbo) -> usize {
        let mut vertices = vec![];
        for (x, col) in layout_cells.iter().enumerate() {
            for (y, cell) in col.iter().enumerate() {
                if let Some(cell) = cell {
                    vertices.extend(cell.create_render_data(Vector2::new(x, y)));
                }
            }
        }

        if vertices.is_empty() {
            return 0;
        }

        unsafe {
            gl::BindVertexArray(vao.0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.0);
            {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * size_of::<f32>()) as isize,
                    &vertices[0] as *const f32 as _,
                    gl::DYNAMIC_DRAW,
                );

                gl::VertexAttribPointer(
                    0,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (size_of::<f32>() * 7) as i32,
                    0 as _,
                );
                gl::EnableVertexAttribArray(0);

                gl::VertexAttribPointer(
                    1,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (size_of::<f32>() * 7) as i32,
                    (2 * size_of::<f32>()) as _,
                );
                gl::EnableVertexAttribArray(1);

                gl::VertexAttribPointer(
                    2,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (size_of::<f32>() * 7) as i32,
                    (4 * size_of::<f32>()) as _,
                );
                gl::EnableVertexAttribArray(2);
            }
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        vertices.len()
    }

    pub fn render_cell(
        camera: &Camera,
        resolution: (f32, f32),
        program: &Program<Shader>,
        len_vec_vertices: usize,
        vao: Vao,
        time: f32,
    ) {
        if len_vec_vertices == 0 {
            return;
        }

        unsafe {
            gl::BindVertexArray(vao.0);
            {
                gl::UseProgram(program.id());
                gl::Uniform2f(
                    0,
                    resolution.0,
                    resolution.1,
                );
                gl::Uniform2f(
                    1,
                    camera.position.x,
                    camera.position.y,
                );
                gl::Uniform1f(2, camera.scale);
                gl::Uniform1f(3, time);
                gl::DrawArrays(gl::TRIANGLES, 0, (len_vec_vertices / 7) as i32);
                gl::UseProgram(0);
            }
            gl::BindVertexArray(0);
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            mass: 16.0,
            energy: 256.0,
            color: [0.5; 3],
        }
    }
}
