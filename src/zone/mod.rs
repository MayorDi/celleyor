use crate::{
    control::Camera,
    grid::constants::{SIZE_GRID, SIZE_RENDER_CELL_GRID},
    opengl::prelude::{get_location, Build, GetId, Program, Shader, Vao, Vbo},
};

#[derive(Debug, Clone, Copy)]
pub struct Zone {
    color: (f32, f32, f32),
}

impl Zone {
    pub fn new(color: (f32, f32, f32)) -> Self {
        Self { color }
    }

    pub fn create_render_data(&self, pos: (f32, f32), borders: i32) -> [f32; 48] {
        let (x, y) = (pos.0 * SIZE_RENDER_CELL_GRID, pos.1 * SIZE_RENDER_CELL_GRID);
        let vertices = [
            x,
            y,
            0.0,
            0.0,
            self.color.0,
            self.color.1,
            self.color.2,
            borders as f32,
            x + SIZE_RENDER_CELL_GRID,
            y,
            1.0,
            0.0,
            self.color.0,
            self.color.1,
            self.color.2,
            borders as f32,
            x,
            y + SIZE_RENDER_CELL_GRID,
            0.0,
            1.0,
            self.color.0,
            self.color.1,
            self.color.2,
            borders as f32,
            x,
            y + SIZE_RENDER_CELL_GRID,
            0.0,
            1.0,
            self.color.0,
            self.color.1,
            self.color.2,
            borders as f32,
            x + SIZE_RENDER_CELL_GRID,
            y + SIZE_RENDER_CELL_GRID,
            1.0,
            1.0,
            self.color.0,
            self.color.1,
            self.color.2,
            borders as f32,
            x + SIZE_RENDER_CELL_GRID,
            y,
            1.0,
            0.0,
            self.color.0,
            self.color.1,
            self.color.2,
            borders as f32,
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

    pub fn build_render_program() -> Program<Shader> {
        let vs = Shader::new(
            gl::VERTEX_SHADER,
            include_bytes!("../../res/shaders/zone/zone.vert").to_vec(),
        );
        let fs = Shader::new(
            gl::FRAGMENT_SHADER,
            include_bytes!("../../res/shaders/zone/zone.frag").to_vec(),
        );

        let mut program = Program::new();
        program.push_shader(vs);
        program.push_shader(fs);
        program.build().unwrap();

        program
    }

    pub fn init_render_zones(
        zones: &[[Option<Zone>; SIZE_GRID[0]]; SIZE_GRID[1]],
        vao: Vao,
        vbo: Vbo,
    ) -> usize {
        let mut vertices = vec![];
        for (x, col) in zones.iter().enumerate() {
            for (y, zone) in col.iter().enumerate() {
                if let Some(zone) = zone {
                    let borders = Self::checking_neighbors((x as i32, y as i32), zones);
                    vertices.extend(zone.create_render_data((x as f32, y as f32), borders));
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
                gl::DrawArrays(gl::TRIANGLES, 0, (len_vec_vertices) as i32);
                gl::UseProgram(0);
            }
            gl::BindVertexArray(0);
        }
    }

    fn checking_neighbors(
        pos: (i32, i32),
        zones: &[[Option<Zone>; SIZE_GRID[0]]; SIZE_GRID[1]],
    ) -> i32 {
        use nalgebra::clamp;
        let mut borders = 0;

        let x_clamp = |x| clamp(x, 0, SIZE_GRID[0] as i32 - 1);
        let y_clamp = |y| clamp(y, 0, SIZE_GRID[1] as i32 - 1);

        let (x, y) = (pos.0 as usize, pos.1 as usize);
        let (left, top, right, bottom) = (
            x_clamp(pos.0 - 1),
            y_clamp(pos.1 + 1),
            x_clamp(pos.0 + 1),
            y_clamp(pos.1 - 1),
        );

        if left != pos.0 {
            if zones[left as usize][y].is_some() {
                borders |= 0b0001;
            }
        }

        if top != pos.1 {
            if zones[x][top as usize].is_some() {
                borders |= 0b0010;
            }
        }

        if right != pos.0 {
            if zones[right as usize][y].is_some() {
                borders |= 0b0100;
            }
        }

        if bottom != pos.1 {
            if zones[x][bottom as usize].is_some() {
                borders |= 0b1000;
            }
        }
        
        borders
    }
}
