use std::collections::HashMap;

use crate::opengl::prelude::{load_bytes_from_file, Build, Program, Shader};

pub fn init_program_shader<'a>() -> HashMap<&'a str, Program<Shader>> {
    let mut programs = HashMap::new();
    programs.insert("zone", build_zone_program_shader());
    programs.insert("grid", build_grid_program_shader());
    programs.insert("cell", build_cell_program_shader());


    programs
}

fn build_zone_program_shader() -> Program<Shader> {
    let vs = Shader::new(
        gl::VERTEX_SHADER,
        load_bytes_from_file("./res/shaders/zone/zone.vert").unwrap(),
    );
    let fs = Shader::new(
        gl::FRAGMENT_SHADER,
        load_bytes_from_file("./res/shaders/zone/zone.frag").unwrap(),
    );

    let mut program = Program::new();
    program.push_shader(vs);
    program.push_shader(fs);
    program.build().unwrap();

    program
}

pub fn build_grid_program_shader() -> Program<Shader> {
    let vs = Shader::new(
        gl::VERTEX_SHADER,
        load_bytes_from_file("./res/shaders/grid/grid.vert").unwrap(),
    );
    let fs = Shader::new(
        gl::FRAGMENT_SHADER,
        load_bytes_from_file("./res/shaders/grid/grid.frag").unwrap(),
    );

    let mut program = Program::new();
    program.push_shader(vs);
    program.push_shader(fs);
    program.build().unwrap();

    program
}

pub fn build_cell_program_shader() -> Program<Shader> {
    let vs = Shader::new(
        gl::VERTEX_SHADER,
        load_bytes_from_file("./res/shaders/cell/cell.vert").unwrap(),
    );
    let fs = Shader::new(
        gl::FRAGMENT_SHADER,
        load_bytes_from_file("./res/shaders/cell/cell.frag").unwrap(),
    );

    let mut program = Program::new();
    program.push_shader(vs);
    program.push_shader(fs);
    program.build().unwrap();

    program
}
