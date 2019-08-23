use crate::instruction::Instruction;
use crate::state::State;
use glium;
use imgui::*;
use imgui_glium_renderer::Renderer;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq)]
pub enum UiAction {
    None,
    Run,
    Stop,
    Step,
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct MouseState {
    pub pos: [f32; 2],
    pub pressed: [bool; 5],
    pub wheel: f32,
}

pub struct Gui {
    imgui: imgui::Context,
    renderer: Renderer,
    game_display_texture_id: imgui::TextureId,
    pub ui_action: UiAction,
}

impl Gui {
    pub fn new(display: &glium::Display) -> Self {
        let mut imgui = imgui::Context::create();
        let renderer = Renderer::init(&mut imgui, display).expect("Failed to initialize renderer!");
        let gl_window = display.gl_window();
        let window = gl_window.window();
        let size_pixels = window.get_inner_size().unwrap();
        imgui.io_mut().display_size = [size_pixels.width as f32, size_pixels.height as f32];

        Gui {
            imgui: imgui,
            renderer: renderer,
            game_display_texture_id: imgui::TextureId::from(0),
            ui_action: UiAction::None,
        }
    }

    pub fn render(
        &mut self,
        target: &mut glium::Frame,
        state: &State,
        game_display: glium::Texture2d,
    ) {
        // Draw GUI
        self.renderer
            .textures()
            .replace(self.game_display_texture_id, Rc::new(game_display));
        self.draw_gui(state, target);
    }

    pub fn update_mouse_state(&mut self, mouse_state: &mut MouseState) {
        self.imgui.io_mut().mouse_pos = mouse_state.pos;
        self.imgui.io_mut().mouse_down = mouse_state.pressed;
        self.imgui.io_mut().mouse_wheel = mouse_state.wheel;
        mouse_state.wheel = 0.0;
    }

    fn draw_gui(&mut self, state: &State, target: &mut glium::Frame) {
        let mut ui_action = self.ui_action;
        let game_display_texture_id = self.game_display_texture_id;
        let ui = self.imgui.frame();
        let display_window_style_token = ui.push_style_vars(&[
            StyleVar::WindowPadding([0.0, 0.0]),
            StyleVar::WindowRounding(0.0),
            StyleVar::WindowBorderSize(0.0),
        ]);
        ui.window(im_str!("Display"))
            .title_bar(false)
            .resizable(false)
            .size([400.0, 200.0], imgui::Condition::Always)
            .build(|| {
                Image::new(&ui, game_display_texture_id, [400.0, 200.0]).build();
            });

        std::mem::drop(display_window_style_token);

        ui.window(im_str!("Registers"))
            .size([100.0, 0.0], imgui::Condition::Always)
            .build(|| {
                for i in 0..15 {
                    ui.text(im_str!("V{:01X}: {:02X}", i, state.v[i]));
                }
                ui.separator();
                ui.text(im_str!("I: {:04X}", state.i));
                ui.text(im_str!("PC: {:04X}", state.pc));
                ui.text(im_str!("SP: {:02X}", state.sp));
                ui.text(im_str!("DT: {:02X}", state.dt));
                ui.text(im_str!("ST: {:02X}", state.st));
            });

        ui.window(im_str!("Stack"))
            .size([100.0, 0.0], imgui::Condition::Always)
            .build(|| {
                for i in 0..15 {
                    ui.text(im_str!("{:01X}: {:04X}", i, state.stack[i]));
                }
            });

        ui.window(im_str!("Control"))
            .size([0.0, 0.0], imgui::Condition::Always)
            .build(|| {
                ui_action = UiAction::None;
                let mut x = 8f32;
                if ui.button(im_str!("Run"), [0.0, 20.0]) {
                    ui_action = UiAction::Run;
                }
                x += ui.get_item_rect_size()[0] + 8.0;
                ui.same_line(x);
                if ui.button(im_str!("Stop"), [0.0, 20.0]) {
                    ui_action = UiAction::Stop;
                }
                x += ui.get_item_rect_size()[0] + 8.0;
                ui.same_line(x);
                if ui.button(im_str!("Step"), [0.0, 20.0]) {
                    ui_action = UiAction::Step;
                }
            });

        ui.window(im_str!("Code"))
            .size([0.0, 0.0], imgui::Condition::Always)
            .build(|| {
                for i in (0x200..(state.ram.len() - 1)).step_by(2) {
                    let _token: ColorStackToken;
                    if i == state.pc as usize {
                        _token = ui.push_style_colors(&[(StyleColor::Text, [1.0, 0.0, 0.0, 1.0])]);
                    }

                    let instruction =
                        Instruction::new(((state.ram[i]) as u16) << 8 | state.ram[i + 1] as u16);
                    ui.text(im_str!(
                        "{:04X}: {} ({:04X})",
                        i,
                        instruction.code,
                        instruction.opcode
                    ));
                }
            });

        self.ui_action = ui_action;

        self.renderer
            .render(target, ui.render())
            .expect("Rendering failed!");
    }
}
