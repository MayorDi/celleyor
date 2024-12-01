use crate::opengl::prelude::{load_bytes_from_file, Build, Program, Shader};

proc_opengl::init_program_shader! {
    zone {
        vs: "./res/shaders/zone/zone.vert",
        fs: "./res/shaders/zone/zone.frag"
    }
    grid {
        vs: "./res/shaders/grid/grid.vert",
        fs: "./res/shaders/grid/grid.frag"
    }
    cell {
        vs: "./res/shaders/cell/cell.vert",
        fs: "./res/shaders/cell/cell.frag"
    }
}
