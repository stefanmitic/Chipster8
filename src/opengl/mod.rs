use crate::state::State;
use glium::glutin;

static PIXELSIZE_X: f32 = 2.0 / 64.0;
static PIXELSIZE_Y: f32 = 2.0 / 32.0;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

implement_vertex!(Vertex, position, color);

pub fn create_window() -> (glium::Display, glutin::EventsLoop) {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Chipster8")
        .with_dimensions(glutin::dpi::LogicalSize::new(800.0, 400.0));
    let window_context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, window_context, &events_loop).unwrap();

    (display, events_loop)
}

pub fn generate_program(display: &glium::Display) -> glium::Program {
    let vertex_shader_src = include_str!("shaders/display.vert");
    let fragment_shader_src = include_str!("shaders/display.frag");
    glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}

pub fn generate_display(state: &State) -> std::vec::Vec<Vertex> {
    let mut vertices = std::vec::Vec::new();
    let mut color: [f32; 4];
    for (row_no, row) in state.display.data.iter().enumerate() {
        for (pixel_no, pixel) in row.iter().enumerate() {
            let x = PIXELSIZE_X * pixel_no as f32 - 1.0;
            let y = PIXELSIZE_Y * row_no as f32 - 1.0;

            if *pixel > 0 {
                color = [1.0, 1.0, 1.0, 1.0];
            } else {
                color = [0.0, 0.0, 0.0, 0.0];
            }

            vertices.push(Vertex {
                position: [x, y],
                color: color,
            });
            vertices.push(Vertex {
                position: [x + PIXELSIZE_X, y],
                color: color,
            });
            vertices.push(Vertex {
                position: [x, y + PIXELSIZE_Y],
                color: color,
            });

            vertices.push(Vertex {
                position: [x + PIXELSIZE_X, y],
                color: color,
            });
            vertices.push(Vertex {
                position: [x + PIXELSIZE_X, y + PIXELSIZE_Y],
                color: color,
            });
            vertices.push(Vertex {
                position: [x, y + PIXELSIZE_Y],
                color: color,
            });
        }
    }
    vertices
}
