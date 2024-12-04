#![warn(temporary_cstring_as_ptr)]
//! Этот модуль предоставляет набор инструментов для упрощения работы с `OpenGL`.

mod load;
mod program;
mod shader;
mod traits;
mod types;

pub mod prelude;

#[macro_export]
macro_rules! uniform {
    ($program:ident: $value:expr => $u_name:literal) => {
        gl::Uniform1f(get_location($program, $u_name), $value);
    };
}
