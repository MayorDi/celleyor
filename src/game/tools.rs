use crate::{
    cell::Cell,
    control::{Camera, Mouse},
    opengl::prelude::{get_location, GetId, Program, Shader},
    zone::Zone,
};

#[derive(Debug, Default)]
pub struct Tools {
    pub select_tools: SelectTools,
    pub zone: Zone,
    pub cell: Cell,
}

#[derive(Debug, Default, PartialEq)]
pub enum SelectTools {
    #[default]
    None,
    AddNewZone,
    AddNewCell,
}

impl Tools {
    pub fn render_ui(&mut self, ui: &mut egui_glfw::egui::Ui) {
        match self.select_tools {
            SelectTools::None => {}
            SelectTools::AddNewZone => {
                ui.color_edit_button_rgb(&mut self.zone.color);
            }
            SelectTools::AddNewCell => {
                ui.color_edit_button_rgb(&mut self.cell.color);
            }
        }
    }

    pub fn is_zone_to_render_zone(
        &self,
        camera: &Camera,
        resolution: (f32, f32),
        mouse: &Mouse,
        program: &Program<Shader>,
    ) {
        if let SelectTools::AddNewZone = self.select_tools {
            let zone = self.zone;

            let (zone_vao, zone_vbo) = Zone::create_render_info();
            let vertices_zone = zone.create_render_data(
                (mouse.grid_position.x as f32, mouse.grid_position.y as f32),
                0,
            );

            unsafe {
                gl::BindVertexArray(zone_vao.0);
                gl::BindBuffer(gl::ARRAY_BUFFER, zone_vbo.0);
                {
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (vertices_zone.len() * size_of::<f32>()) as isize,
                        &vertices_zone[0] as *const f32 as _,
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
                    gl::DrawArrays(gl::TRIANGLES, 0, (vertices_zone.len() / 8) as _);
                }
                gl::BindVertexArray(0);

                gl::DeleteVertexArrays(1, &zone_vao.0);
                gl::DeleteBuffers(1, &zone_vbo.0);
            }
        }
    }

    pub fn is_cell_to_render_cell(
        &self,
        camera: &Camera,
        resolution: (f32, f32),
        mouse: &Mouse,
        program: &Program<Shader>,
    ) {
        if let SelectTools::AddNewCell = self.select_tools {
            let cell = &self.cell;

            let (cell_vao, cell_vbo) = Cell::create_render_info();
            let vertices = cell.create_render_data(mouse.grid_position);

            unsafe {
                gl::BindVertexArray(cell_vao.0);
                gl::BindBuffer(gl::ARRAY_BUFFER, cell_vbo.0);
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
                    gl::Uniform1f(get_location(program, "u_time"), 0.0);
                    gl::DrawArrays(gl::TRIANGLES, 0, (vertices.len() / 7) as _);
                }
                gl::BindVertexArray(0);

                gl::DeleteVertexArrays(1, &cell_vao.0);
                gl::DeleteBuffers(1, &cell_vbo.0);
            }
        }
    }
}
