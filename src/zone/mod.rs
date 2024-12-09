use crate::{
    control::Camera,
    grid::{
        constants::{SIZE_GRID, SIZE_RENDER_CELL_GRID},
        layout::{idx_to_pos, pos_to_idx, Layout},
    },
    opengl::prelude::{GetId, Program, Shader, Vao, Vbo},
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Zone {
    pub(crate) color: [f32; 3],
}

impl Zone {
    pub fn create_render_data(&self, pos: nalgebra::Vector2<usize>, borders: i32) -> [f32; 48] {
        let borders = borders as f32;
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
            borders,
            x + SIZE_RENDER_CELL_GRID,
            y,
            1.0,
            0.0,
            self.color[0],
            self.color[1],
            self.color[2],
            borders,
            x,
            y + SIZE_RENDER_CELL_GRID,
            0.0,
            1.0,
            self.color[0],
            self.color[1],
            self.color[2],
            borders,
            x,
            y + SIZE_RENDER_CELL_GRID,
            0.0,
            1.0,
            self.color[0],
            self.color[1],
            self.color[2],
            borders,
            x + SIZE_RENDER_CELL_GRID,
            y + SIZE_RENDER_CELL_GRID,
            1.0,
            1.0,
            self.color[0],
            self.color[1],
            self.color[2],
            borders,
            x + SIZE_RENDER_CELL_GRID,
            y,
            1.0,
            0.0,
            self.color[0],
            self.color[1],
            self.color[2],
            borders,
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

    pub fn init_render_zones(zones: &Layout<Zone>, vao: Vao, vbo: Vbo) -> usize {
        if !zones.is_need_render {
            return 0;
        }

        let mut vertices = vec![];
        for i in 0..zones.len() {
            if let Some(zone) = zones[i] {
                let borders = Self::checking_neighbors(i, zones);
                vertices.extend(zone.create_render_data(idx_to_pos(i), borders));
            }
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
                    (size_of::<f32>() * 8) as i32,
                    0 as _,
                );
                gl::EnableVertexAttribArray(0);

                gl::VertexAttribPointer(
                    1,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (size_of::<f32>() * 8) as i32,
                    (2 * size_of::<f32>()) as _,
                );
                gl::EnableVertexAttribArray(1);

                gl::VertexAttribPointer(
                    2,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (size_of::<f32>() * 8) as i32,
                    (4 * size_of::<f32>()) as _,
                );
                gl::EnableVertexAttribArray(2);

                gl::VertexAttribPointer(
                    3,
                    1,
                    gl::FLOAT,
                    gl::FALSE,
                    (size_of::<f32>() * 8) as i32,
                    (7 * size_of::<f32>()) as _,
                );
                gl::EnableVertexAttribArray(3);
            }
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        vertices.len()
    }

    pub fn render_zone(
        camera: &Camera,
        resolution: (f32, f32),
        program: &Program<Shader>,
        len_vec_vertices: usize,
        vao: Vao,
    ) {
        if len_vec_vertices == 0 {
            return;
        }

        unsafe {
            gl::BindVertexArray(vao.0);
            {
                gl::UseProgram(program.id());
                gl::Uniform2f(0, resolution.0, resolution.1);
                gl::Uniform2f(1, camera.position.x, camera.position.y);
                gl::Uniform1f(2, camera.scale);
                gl::DrawArrays(gl::TRIANGLES, 0, (len_vec_vertices / 8) as i32);
                gl::UseProgram(0);
            }
            gl::BindVertexArray(0);
        }
    }

    fn checking_neighbors(index: usize, zones: &Layout<Zone>) -> i32 {
        use nalgebra::clamp;
        let mut borders = 0;

        let x_clamp = |x| clamp(x, 0, SIZE_GRID[0] as i32 - 1);
        let y_clamp = |y| clamp(y, 0, SIZE_GRID[1] as i32 - 1);

        let pos = idx_to_pos(index);
        let (x, y) = (pos.x as i32, pos.y as i32);
        let (left, top, right, bottom) = (
            pos_to_idx(nalgebra::Vector2::new(x_clamp(x - 1) as usize, pos.y)),
            pos_to_idx(nalgebra::Vector2::new(pos.x, y_clamp(y + 1) as usize)),
            pos_to_idx(nalgebra::Vector2::new(x_clamp(x + 1) as usize, pos.y)),
            pos_to_idx(nalgebra::Vector2::new(pos.x, y_clamp(y - 1) as usize)),
        );

        let idx = pos_to_idx(pos);
        if left != idx && zones[left].is_some() {
            borders |= 0b0001;
        }

        if top != idx && zones[top].is_some() {
            borders |= 0b0010;
        }

        if right != idx && zones[right].is_some() {
            borders |= 0b0100;
        }

        if bottom != idx && zones[bottom].is_some() {
            borders |= 0b1000;
        }

        borders
    }
}
