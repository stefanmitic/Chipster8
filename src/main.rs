extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;

use std::env;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path;
use std::time::Duration;

mod display;
mod instruction;
mod state;

use instruction::Instruction;
use state::State;

static PIXELSIZE: u32 = 5;

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

fn draw_display(canvas: &mut sdl2::render::Canvas<Window>, state: &State) {
    for (row_no, row) in state.display.data.iter().enumerate() {
        for (pixel_no, pixel) in row.iter().enumerate() {
            let x = PIXELSIZE * pixel_no as u32;
            let y = PIXELSIZE * row_no as u32;
            if *pixel > 0 {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            match canvas.fill_rect(Rect::new(x as i32, y as i32, x + PIXELSIZE, y + PIXELSIZE)) {
                Ok(ok) => ok,
                Err(err) => panic!(err),
            }
        }
    }
}

fn execute(state: &mut State) -> bool {
    let instruction = Instruction::new(
        ((state.ram[state.pc as usize]) as u16) << 8 | state.ram[(state.pc + 1) as usize] as u16,
    );

    if !instruction.function(state) {
        println!("Failed to execute instruction!");
        return false;
    }
    state.pc += 2;
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
    let mut cycles_passed = 0;
    let mut state: State = State::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: chipster8 path_to_rom");
        return;
    }
    load_program(path::Path::new(&args[1]), &mut state);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chipster8", PIXELSIZE * 64, PIXELSIZE * 32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        execute(&mut state);
        if cycles_passed >= 10 {
            update_timers(&mut state);
            cycles_passed = 0;
        } else {
            cycles_passed += 1;
        }
        draw_display(&mut canvas, &state);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
    }
}
