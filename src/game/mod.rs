use components::{EguiComponents, WindowComponents};
use egui_glfw::{self as egui_backend, EguiInputState};

use egui_backend::egui::{self, vec2, Pos2, Rect};
use egui_glfw::glfw::Context;
use glfw::{Glfw, Window};

use crate::{
    control::{Camera, Mouse},
    grid::Grid,
    zone::Zone,
};

mod components;
mod tools;

pub struct Game {
    window_components: WindowComponents,
    egui_components: EguiComponents,
}

impl Game {
    pub fn init() -> Self {
        let mut wc = Self::init_window_components();

        gl::load_with(|symbol| wc.window.get_proc_address(symbol) as *const _);
        wc.window.set_framebuffer_size_callback(|_, w, h| unsafe {
            gl::Viewport(0, 0, w, h);
        });
        wc.window.set_all_polling(true);
        wc.window.make_current();

        let egui_c = Self::init_gui_components(&mut wc.window);

        Self {
            window_components: wc,
            egui_components: egui_c,
        }
    }

    fn init_window_components() -> WindowComponents {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        Self::init_window_hints(&mut glfw);

        let (window, events) = glfw
            .create_window(1200, 600, "Celleyor", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        WindowComponents {
            glfw,
            window,
            events,
        }
    }

    fn init_window_hints(glfw: &mut Glfw) {
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(true));
    }

    fn init_gui_components(window: &mut Window) -> EguiComponents {
        let painter = egui_backend::Painter::new(window);
        let egui_ctx = egui::Context::default();

        let (width, height) = window.get_framebuffer_size();
        let native_pixels_per_point = window.get_content_scale().0;

        let mut egui_input_state =
            Self::create_egui_input_state(width as f32, height as f32, native_pixels_per_point);

        egui_input_state.input.time = Some(0.01);

        EguiComponents {
            painter,
            egui_ctx,
            native_pixels_per_point,
            egui_input_state,
        }
    }

    fn create_egui_input_state(
        width: f32,
        height: f32,
        native_pixels_per_point: f32,
    ) -> EguiInputState {
        EguiInputState::new(egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0f32, 0f32),
                vec2(width, height) / native_pixels_per_point,
            )),
            ..Default::default()
        })
    }

    pub fn run(self) {
        let WindowComponents {
            mut glfw,
            mut window,
            events,
        } = self.window_components;

        let mut egui_components = self.egui_components;

        let mut camera = Camera::new();
        let mut mouse = Mouse::new();

        let mut grid = Grid::new();
        let render_program_grid = Grid::build_render_program();
        let (grid_vao, _) = grid.create_render_info();

        let render_program_zones = Zone::build_render_program();
        let (zone_vao, zone_vbo) = Zone::create_render_info();

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        while !window.should_close() {
            let (w, h) = window.get_size();
            let resolution = (w as f32, h as f32);

            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        window.set_should_close(true);
                    }

                    glfw::WindowEvent::MouseButton(button, action, _) => {
                        mouse.button = button;

                        match action {
                            glfw::Action::Press => mouse.pressed = true,
                            _ => mouse.pressed = false,
                        }
                    }

                    glfw::WindowEvent::Scroll(_, y) => {
                        if (camera.scale + y as f32) > 0.0 {
                            camera.scale += y as f32 / 2.0;
                        }
                    }

                    glfw::WindowEvent::CursorPos(x, y) => {
                        mouse.old_position = mouse.position;
                        mouse.position = nalgebra::Vector2::new(x as f32, y as f32);
                        mouse.world_position = nalgebra::Vector2::new(
                            camera.position.x / 2.0
                                + (mouse.position.x - resolution.0 / 2.0) / camera.scale,
                            camera.position.y / 2.0
                                + (resolution.1 / 2.0 - mouse.position.y) / camera.scale,
                        );

                        if mouse.pressed {
                            match mouse.button {
                                glfw::MouseButton::Button3 => {
                                    camera.position += mouse.delta() / camera.scale;
                                }
                                _ => {}
                            }
                        }
                    }

                    _ => {}
                }

                match event {
                    _ => {
                        egui_backend::handle_event(event, &mut egui_components.egui_input_state);
                    }
                }
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);

                grid.render_grid(&camera, resolution, &render_program_grid, grid_vao);

                let len_vec_vertices =
                    Zone::init_render_zones(&grid.layout_zones, zone_vao, zone_vbo);

                Zone::render_zone(
                    &camera,
                    resolution,
                    &render_program_zones,
                    len_vec_vertices,
                    zone_vao,
                );
            }

            Self::render_ui(&mut egui_components, &mouse);

            window.swap_buffers();
        }
    }

    fn create_ui(ctx: &egui::Context, mouse: &Mouse) {
        egui::SidePanel::new(egui::containers::panel::Side::Right, "panel_tools").show(ctx, |ui| {
            ui.heading("Celleyor");
            ui.separator();
        });

        egui::TopBottomPanel::bottom("info_panel").show(ctx, |ui| {
            ui.label(format!(
                "world mouse pos: [x:\t{:.2}; y:\t{:.2}]",
                mouse.world_position.x, mouse.world_position.y
            ))
        });
    }

    fn render_ui(egui_components: &mut EguiComponents, mouse: &Mouse) {
        let EguiComponents {
            egui_ctx,
            painter,
            native_pixels_per_point,
            egui_input_state,
        } = egui_components;

        egui_ctx.begin_frame(egui_input_state.input.take());
        Self::create_ui(egui_ctx, mouse);

        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            ..
        } = egui_ctx.end_frame();

        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(egui_input_state, platform_output.copied_text);
        }

        let clipped_shapes = egui_ctx.tessellate(shapes, *native_pixels_per_point);
        painter.paint_and_update_textures(
            &egui_ctx,
            *native_pixels_per_point,
            &clipped_shapes,
            &textures_delta,
        );
    }
}
