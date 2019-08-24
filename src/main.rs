#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate rand;

use glium::glutin::{
    dpi::LogicalPosition, ElementState::Pressed, Event::WindowEvent, MouseButton, MouseScrollDelta,
    TouchPhase, WindowEvent::*,
};
use std::env;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path;
use std::time::Duration;
use std::time::Instant;

mod display;
mod gui;
mod instruction;
mod opengl;
mod state;

use gui::{Gui, MouseState, UiAction};
use instruction::Instruction;
use state::State;

fn load_program(path: &path::Path, state: &mut state::State) {
    let mut file = match fs::File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };
    let file_size = fs::metadata(path).unwrap().len();

    let mut buffer = vec![0u8; file_size as usize];
    let bytes_read = match file.read(&mut buffer) {
        Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
        Ok(bytes_read) => bytes_read,
    };

    if bytes_read != file_size as usize {
        panic!(
            "File size and bytes read missmatch! {} vs {}",
            file_size, bytes_read
        );
    }

    println!("Read file: {} Total bytes: {}", path.display(), bytes_read);

    state.ram[0x200..(0x200 + bytes_read)].clone_from_slice(&buffer[0..]);
}

fn execute(state: &mut State) -> bool {
    let instruction = Instruction::new(
        ((state.ram[state.pc as usize]) as u16) << 8 | state.ram[(state.pc + 1) as usize] as u16,
    );

    if !instruction.function(state) {
        println!("Failed to execute instruction!");
        return false;
    }
    true
}

fn update_timers(state: &mut State) {
    if state.dt > 0 {
        state.dt -= 1;
    }

    if state.st > 0 {
        state.st -= 1;
    }
}

fn main() {
    use glium::Surface;
    let mut state: State = State::new();
    let mut mouse_state = MouseState::default();
    let (display, mut events_loop) = opengl::create_window();
    let mut gui: Gui = Gui::new(&display);

    let mut last_frame = Instant::now();
    let mut closed = false;
    let mut simmulation_running = false;
    let mut simmulation_step = false;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = opengl::generate_program(&display);
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: chipster8 path_to_rom");
        return;
    }
    load_program(path::Path::new(&args[1]), &mut state);

    while !closed {
        events_loop.poll_events(|event| {
            if let WindowEvent { event, .. } = event {
                match event {
                    CloseRequested => closed = true,
                    CursorMoved {
                        position: LogicalPosition { x, y },
                        ..
                    } => mouse_state.pos = [x as f32, y as f32],
                    MouseInput { state, button, .. } => match button {
                        MouseButton::Left => mouse_state.pressed[0] = state == Pressed,
                        MouseButton::Right => mouse_state.pressed[1] = state == Pressed,
                        MouseButton::Middle => mouse_state.pressed[2] = state == Pressed,
                        _ => {}
                    },
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => mouse_state.wheel = y,
                    MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(pos),
                        phase: TouchPhase::Moved,
                        ..
                    } => mouse_state.wheel = pos.y as f32,
                    _ => (),
                }
            }
        });

        for i in 0..9 {
            if simmulation_running || simmulation_step {
                execute(&mut state);
                if i == 0 {
                    update_timers(&mut state);
                }
                simmulation_step = false;
            }
        }

        gui.update_mouse_state(&mut mouse_state);
        let shape = opengl::generate_display(&state);
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let texture = glium::Texture2d::empty(&display, 400, 200).unwrap();
        texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
        texture
            .as_surface()
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        gui.render(&mut target, &state, texture);
        target.finish().unwrap();

        match gui.ui_action {
            UiAction::Run => simmulation_running = true,
            UiAction::Stop => simmulation_running = false,
            UiAction::Step => {
                simmulation_running = false;
                simmulation_step = true;
            }
            UiAction::None => (),
        }

        let now = Instant::now();
        let delta = now - last_frame;
        last_frame = now;

        if delta < Duration::from_millis(16) {
            ::std::thread::sleep(Duration::from_millis(16) - delta);
        }
    }
}
