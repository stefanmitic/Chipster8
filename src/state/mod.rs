use crate::display::Display;
use std::fmt;

// #[derive(Debug)]
pub struct State {
    pub i: u16,
    pub pc: u16,
    pub sp: u8,
    pub dt: u8,
    pub st: u8,
    pub v: [u8; 16],
    pub stack: [u16; 16],
    pub keypad: [bool; 16],
    pub display: Display,
    pub ram: [u8; 4095],
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("State")
            .field("i", &self.i)
            .field("pc", &self.pc)
            .field("sp", &self.sp)
            .field("dt", &self.dt)
            .field("st", &self.st)
            .field("v", &self.v)
            .field("stack", &self.stack)
            .field("keypad", &self.keypad)
            .field("display", &format_args!("\n{:?}", &self.display))
            .finish()
    }
}

impl State {
    pub fn new() -> State {
        State {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            dt: 0,
            st: 0,
            display: Display::new(),
            keypad: [false; 16],
            ram: [0; 0xFFF],
        }
        .fill_ram()
    }

    fn fill_ram(mut self) -> State {
        let character_data = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        self.ram[0..80].copy_from_slice(&character_data);

        self
    }

    pub fn push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn push_test() {
        let mut state = State::new();

        state.push(0xABC);
        assert_eq!(0, state.sp - 1);
        assert_eq!(0xABC, state.stack[(state.sp - 1) as usize]);
    }

    #[test]
    fn pop_test() {
        let mut state = State::new();

        state.stack[0] = 0xABC;
        state.sp = 1;
        assert_eq!(0xABC, state.pop());
        assert_eq!(0, state.sp);
    }
}
