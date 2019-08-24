use crate::state::State;
use rand::Rng;
use std::fmt;

fn get_x(opcode: u16) -> u16 {
    (opcode & 0x0F00) >> 8
}

fn get_y(opcode: u16) -> u16 {
    (opcode & 0x00F0) >> 4
}

fn get_nnn(opcode: u16) -> u16 {
    opcode & 0x0FFF
}

fn get_addr(opcode: u16) -> u16 {
    opcode & 0x0FFF
}

fn get_nibble(opcode: u16) -> u16 {
    opcode & 0x000F
}

fn get_byte(opcode: u16) -> u8 {
    (opcode & 0x00FF) as u8
}

pub struct Instruction {
    pub opcode: u16,
    pub code: String,
    pub function: Box<dyn Fn(u16, &mut State) -> bool>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04X}", self.opcode)
    }
}

impl Instruction {
    pub fn new(opcode: u16) -> Instruction {
        let opcode_single_id = opcode & 0xF000;
        let opcode_double_id = opcode & 0xF00F;
        let opcode_tripple_id = opcode & 0xF0FF;

        match opcode_single_id {
            0x0000 => match opcode {
                // 0x00E0 - CLS
                0x00E0 => Instruction {
                    opcode: opcode,
                    code: String::from("CLS"),
                    function: Box::new(|_opcode, state| {
                        state.display.reset();
                        true
                    }),
                },
                // 0x00EE - RET
                0x00EE => Instruction {
                    opcode: opcode,
                    code: String::from("RET"),
                    function: Box::new(|_opcode, state| {
                        state.pc = state.pop();
                        state.pc += 2;
                        true
                    }),
                },
                // 0nnn - SYS addr
                _ => Instruction {
                    opcode: opcode,
                    code: String::from(format!("SYS {:03X}", get_addr(opcode))),
                    function: Box::new(|opcode, state| {
                        state.pc = get_addr(opcode);
                        true
                    }),
                },
            },
            // 1nnn - JMP addr
            0x1000 => Instruction {
                opcode: opcode,
                code: String::from(format!("JMP {:03X}", get_nnn(opcode))),
                function: Box::new(|opcode, state| {
                    state.pc = get_nnn(opcode);
                    true
                }),
            },
            // 2nnn - CALL addr
            0x2000 => Instruction {
                opcode: opcode,
                code: String::from(format!("CALL {:03X}", get_addr(opcode))),
                function: Box::new(|opcode, state| {
                    state.push(state.pc);
                    state.pc = get_addr(opcode);
                    true
                }),
            },
            // 3xkk - SE Vx, byte
            0x3000 => Instruction {
                opcode: opcode,
                code: String::from(format!(
                    "SE V{:01X}, {:02X}",
                    get_x(opcode),
                    get_byte(opcode)
                )),
                function: Box::new(|opcode, state| {
                    let x = get_x(opcode);
                    let byte = get_byte(opcode);

                    if state.v[x as usize] == byte {
                        state.pc += 2;
                    }
                    state.pc += 2;
                    true
                }),
            },
            // 4xkk - SNE Vx, byte
            0x4000 => Instruction {
                opcode: opcode,
                code: String::from(format!(
                    "SNE V{:01X}, {:02X}",
                    get_x(opcode),
                    get_byte(opcode)
                )),
                function: Box::new(|opcode, state| {
                    let x = get_x(opcode);
                    let byte = get_byte(opcode);

                    if state.v[x as usize] != byte {
                        state.pc += 2;
                    }
                    state.pc += 2;
                    true
                }),
            },
            // 5xkk - SE Vx, Vy
            0x5000 => Instruction {
                opcode: opcode,
                code: String::from(format!("SE V{:01X}, V{:01X}", get_x(opcode), get_y(opcode))),
                function: Box::new(|opcode, state| {
                    let x = get_x(opcode);
                    let y = get_y(opcode);

                    if state.v[x as usize] == state.v[y as usize] {
                        state.pc += 2;
                    }
                    state.pc += 2;
                    true
                }),
            },
            // 6xkk - LD Vx, byte
            0x6000 => Instruction {
                opcode: opcode,
                code: String::from(format!(
                    "LD V{:01X}, {:02X}",
                    get_x(opcode),
                    get_byte(opcode)
                )),
                function: Box::new(|opcode, state| {
                    let x = get_x(opcode);
                    let byte = get_byte(opcode);

                    state.v[x as usize] = byte;
                    state.pc += 2;
                    true
                }),
            },
            // 7xkk - ADD Vx, byte
            0x7000 => Instruction {
                opcode: opcode,
                code: String::from(format!(
                    "ADD V{:01X}, {:02X}",
                    get_x(opcode),
                    get_byte(opcode)
                )),
                function: Box::new(|opcode, state| {
                    let x = get_x(opcode);
                    let byte = get_byte(opcode);

                    state.v[x as usize] = (state.v[x as usize] as u16 + byte as u16) as u8;
                    state.pc += 2;
                    true
                }),
            },
            0x8000 => match opcode_double_id {
                // 8xy0 - LD Vx, Vy
                0x8000 => Instruction {
                    opcode: opcode,
                    code: String::from(format!(
                        "LD V{:01X}, V{:01X}",
                        get_x(opcode),
                        get_y(opcode)
                    )),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        let y = get_y(opcode);

                        state.v[x as usize] = state.v[y as usize];
                        state.pc += 2;
                        true
                    }),
                },
                // 8xy1 - OR Vx, Vy
                0x8001 => Instruction {
                    opcode: opcode,
                    code: String::from(format!(
                        "OR V{:01X}, V{:01X}",
                        get_x(opcode),
                        get_y(opcode)
                    )),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        let y = get_y(opcode);

                        state.v[x as usize] |= state.v[y as usize];
                        state.pc += 2;
                        true
                    }),
                },
                // 8xy2 - AND Vx, Vy
                0x8002 => Instruction {
                    opcode: opcode,
                    code: String::from(format!(
                        "AND V{:01X}, V{:01X}",
                        get_x(opcode),
                        get_y(opcode)
                    )),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        let y = get_y(opcode);

                        state.v[x as usize] &= state.v[y as usize];
                        state.pc += 2;
                        true
                    }),
                },
                // 8xy3 - XOR Vx, Vy
                0x8003 => Instruction {
                    opcode: opcode,
                    code: String::from(format!(
                        "XOR V{:01X}, V{:01X}",
                        get_x(opcode),
                        get_y(opcode)
                    )),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        let y = get_y(opcode);

                        state.v[x as usize] ^= state.v[y as usize];
                        state.pc += 2;
                        true
                    }),
                },
                // 8xy4 - ADD Vx, Vy
                0x8004 => Instruction {
                    opcode: opcode,
                    code: String::from(format!(
                        "ADD V{:01X}, V{:01X}",
                        get_x(opcode),
                        get_y(opcode)
                    )),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        let y = get_y(opcode);
                        let result: u16 = state.v[x as usize] as u16 + state.v[y as usize] as u16;
                        if result > 255 {
                            state.v[15] = 1;
                        } else {
                            state.v[15] = 0;
                        }
                        state.v[x as usize] = (result % 0xFF) as u8;
                        state.pc += 2;
                        true
                    }),
                },
                // 8xy5 - SUB Vx, Vy
                0x8005 => Instruction {
                    opcode: opcode,
                    code: String::from(format!(
                        "SUB V{:01X}, V{:01X}",
                        get_x(opcode),
                        get_y(opcode)
                    )),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        let y = get_y(opcode);

                        if state.v[x as usize] > state.v[y as usize] {
                            state.v[15] = 1;
                        } else {
                            state.v[15] = 0;
                        }

                        let result: i8 = state.v[x as usize] as i8 - state.v[y as usize] as i8;
                        state.v[x as usize] = result as u8;
                        state.pc += 2;
                        true
                    }),
                },
                // 8xy6 - SHR Vx {, Vy}
                0x8006 => Instruction {
                    opcode: opcode,
                    code: String::from(format!("SHR V{:01X}", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);

                        state.v[15] = state.v[x as usize] & 0x01;
                        state.v[x as usize] >>= 1;
                        state.pc += 2;
                        true
                    }),
                },
                // 8xy7 - SUBN Vx, Vy
                0x8007 => Instruction {
                    opcode: opcode,
                    code: String::from(format!(
                        "SUBN V{:01X}, V{:01X}",
                        get_x(opcode),
                        get_y(opcode)
                    )),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        let y = get_y(opcode);

                        if state.v[y as usize] > state.v[x as usize] {
                            state.v[15] = 1;
                        } else {
                            state.v[15] = 0;
                        }

                        let result: i8 = state.v[y as usize] as i8 - state.v[x as usize] as i8;
                        state.v[x as usize] = result as u8;
                        state.pc += 2;
                        true
                    }),
                },
                // 8xyE - SHL Vx {, Vy}
                0x800E => Instruction {
                    opcode: opcode,
                    code: String::from(format!("SHL V{:01X}", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);

                        state.v[15] = (state.v[x as usize] & 0x80) >> 7;
                        state.v[x as usize] <<= 1;
                        state.pc += 2;
                        true
                    }),
                },
                _ => Instruction {
                    opcode: opcode,
                    code: String::from(format!("Unknonw instruction: {:04X}", opcode)),
                    function: Box::new(|opcode, state| {
                        println!("Unknown instruction: {:04X}", opcode);
                        println!("State: {:#?}", state);
                        false
                    }),
                },
            },
            // 9xy0 - SNE Vx, Vy
            0x9000 => Instruction {
                opcode: opcode,
                code: String::from(format!(
                    "SNE V{:01X}, V{:01X}",
                    get_x(opcode),
                    get_y(opcode)
                )),
                function: Box::new(|opcode, state| {
                    let x = get_x(opcode);
                    let y = get_y(opcode);

                    if state.v[x as usize] != state.v[y as usize] {
                        state.pc += 2;
                    }
                    true
                }),
            },
            // Annn - LD I, addr
            0xA000 => Instruction {
                opcode: opcode,
                code: String::from(format!("LD I, {:03X}", get_addr(opcode))),
                function: Box::new(|opcode, state| {
                    let addr = get_addr(opcode);

                    state.i = addr;
                    state.pc += 2;
                    true
                }),
            },
            // Bnnn - JP V0, addr
            0xB000 => Instruction {
                opcode: opcode,
                code: String::from(format!("JP V0, {:03X}", get_addr(opcode))),
                function: Box::new(|opcode, state| {
                    let addr = get_addr(opcode);

                    state.pc = state.v[0] as u16 + addr;
                    true
                }),
            },
            // Cxkk - RND Vx, byte
            0xC000 => Instruction {
                opcode: opcode,
                code: String::from(format!(
                    "RND V{:01X}, {:02X}",
                    get_x(opcode),
                    get_byte(opcode)
                )),
                function: Box::new(|opcode, state| {
                    let x = get_x(opcode);
                    let byte = get_byte(opcode);

                    state.v[x as usize] = rand::thread_rng().gen_range(0, 256) as u8 & byte;
                    state.pc += 2;
                    true
                }),
            },
            // Dxyn - DRW Vx, Vy, nibble
            0xD000 => Instruction {
                opcode: opcode,
                code: String::from(format!(
                    "DRW V{:01X}, V{:01X}, {:01X}",
                    get_x(opcode),
                    get_x(opcode),
                    get_nibble(opcode)
                )),
                function: Box::new(|opcode, state| {
                    let x = get_x(opcode);
                    let y = get_y(opcode);
                    let nibble = get_nibble(opcode);
                    let sprite = &state.ram[(state.i as usize)..(state.i + nibble) as usize];

                    state.v[15] = state.display.display_sprite(
                        state.v[x as usize],
                        state.v[y as usize],
                        sprite,
                    ) as u8;

                    state.pc += 2;
                    true
                }),
            },
            0xE000 => match opcode_tripple_id {
                // Ex9E - SKP Vx
                0xE09E => Instruction {
                    opcode: opcode,
                    code: String::from(format!("SKP V{:01X}", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        if state.keypad[state.v[x as usize] as usize] {
                            state.pc += 2;
                        }
                        true
                    }),
                },
                // ExA1 - SKNP Vx
                0xE0A1 => Instruction {
                    opcode: opcode,
                    code: String::from(format!("SKNP V{:01X}", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        if !state.keypad[state.v[x as usize] as usize] {
                            state.pc += 2;
                        }
                        true
                    }),
                },
                _ => Instruction {
                    opcode: opcode,
                    code: String::from(format!("Unknonw instruction: {:04X}", opcode)),
                    function: Box::new(|opcode, state| {
                        println!("Unknown instruction: {:04X}", opcode);
                        println!("State: {:#?}", state);
                        false
                    }),
                },
            },
            0xF000 => match opcode_tripple_id {
                // Fx07 - LD Vx, DT
                0xF007 => Instruction {
                    opcode: opcode,
                    code: String::from(format!("LD V{:01X}, DT", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        state.v[x as usize] = state.dt;
                        state.pc += 2;
                        true
                    }),
                },
                // Fx0A - LD Vx, K
                0xF00A => Instruction {
                    opcode: opcode,
                    code: String::from(format!("LD V{:01X}, K", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        for (i, k) in state.keypad.iter().enumerate() {
                            if *k {
                                state.v[x as usize] = i as u8;
                                state.pc += 2;
                            }
                        }
                        true
                    }),
                },
                // Fx15 - LD DT, Vx
                0xF015 => Instruction {
                    opcode: opcode,
                    code: String::from(format!("LD DT, V{:01X}", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        state.dt = state.v[x as usize];
                        state.pc += 2;
                        true
                    }),
                },
                // Fx18 - LD ST, Vx
                0xF018 => Instruction {
                    opcode: opcode,
                    code: String::from(format!("LD ST, V{:01X}", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        state.st = state.v[x as usize];
                        state.pc += 2;
                        true
                    }),
                },
                // Fx1E - ADD I, Vx
                0xF01E => Instruction {
                    opcode: opcode,
                    code: String::from(format!("ADD I, V{:01X}", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        state.i += state.v[x as usize] as u16;
                        state.pc += 2;
                        true
                    }),
                },
                // Fx29 - LD F, Vx
                0xF029 => Instruction {
                    opcode: opcode,
                    code: String::from(format!("LD F, V{:01X}", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        state.i = (state.v[x as usize] * 5) as u16; // Sprites are 8 x 5
                        state.pc += 2;
                        true
                    }),
                },
                // Fx33 - LD B, Vx
                0xF033 => Instruction {
                    opcode: opcode,
                    code: String::from(format!("LD B, V{:01X}", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        let mut data = state.v[x as usize];
                        for i in (0..3).rev() {
                            state.ram[(state.i + i) as usize] = data % 10;
                            data /= 10;
                        }
                        state.pc += 2;
                        true
                    }),
                },
                // Fx55 - LD [I], Vx
                0xF055 => Instruction {
                    opcode: opcode,
                    code: String::from(format!("LD [I], V{:01X}", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        for i in 0..(x + 1) {
                            state.ram[(state.i + i) as usize] = state.v[i as usize];
                        }
                        state.pc += 2;
                        true
                    }),
                },
                // Fx65 - LD Vx, [I]
                0xF065 => Instruction {
                    opcode: opcode,
                    code: String::from(format!("LD V{:01X}, [I]", get_x(opcode))),
                    function: Box::new(|opcode, state| {
                        let x = get_x(opcode);
                        for i in 0..(x + 1) {
                            state.v[i as usize] = state.ram[(state.i + i) as usize];
                        }
                        state.pc += 2;
                        true
                    }),
                },
                _ => Instruction {
                    opcode: opcode,
                    code: String::from(format!("Unknonw instruction: {:04X}", opcode)),
                    function: Box::new(|opcode, state| {
                        println!("Unknown instruction: {:04X}", opcode);
                        println!("State: {:#?}", state);
                        false
                    }),
                },
            },
            _ => Instruction {
                opcode: opcode,
                code: String::from(format!("Unknonw instruction: {:04X}", opcode)),
                function: Box::new(|opcode, state| {
                    println!("Unknown instruction: {:04X}", opcode);
                    println!("State: {:#?}", state);
                    false
                }),
            },
        }
    }

    pub fn parse_chunk(data: Vec<u16>) -> Vec<Instruction> {
        let mut program: Vec<Instruction> = Vec::with_capacity(data.len());
        for opcode in data {
            program.push(Instruction::new(opcode));
        }

        program
    }

    pub fn function(&self, state: &mut State) -> bool {
        (self.function)(self.opcode, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // 0nnn - SYS addr
    fn sys() {
        let mut state = State::new();
        let instruction = Instruction::new(0x0ABC);
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xABC, state.pc);
    }

    #[test]
    // 00EE - RET
    fn ret() {
        let mut state = State::new();
        let instruction = Instruction::new(0x00EE);
        state.pc = 0xA;
        state.push(0xB);

        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xB, state.pc);
        assert_eq!(0, state.sp);
    }

    #[test]
    // 00E0 - CLS
    fn cls() {
        let mut state = State::new();
        let instruction = Instruction::new(0x00E0);
        let sprite = [0xF0, 0x90, 0x90, 0x90, 0xF0];

        state.display.display_sprite(0, 0, &sprite);

        assert_eq!(false, state.display.is_clear());
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(true, state.display.is_clear());
    }

    #[test]
    // 1nnn - JMP addr
    fn jmp() {
        let mut state = State::new();
        let instruction = Instruction::new(0x1ABC);

        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xABC, state.pc);
    }

    #[test]
    // 2nnn - CALL addr
    fn call() {
        let mut state = State::new();
        let instruction = Instruction::new(0x2ABC);

        state.pc = 0xAAA;

        assert_eq!(0, state.sp);
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xABC, state.pc);
        assert_eq!(0xAAA, state.pop());
    }

    #[test]
    // 3xkk - SE Vx, byte
    fn se_vx_byte() {
        let mut state = State::new();
        let instruction = Instruction::new(0x31AA); // V1 == 0xAA

        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x200, state.pc);

        state.v[1] = 0xAA;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x202, state.pc);
    }

    #[test]
    // 4xkk - SNE, Vx, byte
    fn sne_vx_byte() {
        let mut state = State::new();
        let instruction = Instruction::new(0x41AA); // V1 != 0xAA

        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x202, state.pc);

        state.v[1] = 0xAA;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x202, state.pc);
    }

    #[test]
    // 5xy0 - SE Vx, Vy
    fn se_vx_vy() {
        let mut state = State::new();
        let instruction = Instruction::new(0x5010); // V0 == V1

        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x202, state.pc);

        state.v[0] = 0xAA;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x202, state.pc);
    }

    #[test]
    // 6xkk - LD Vx, byte
    fn ld_vx_byte() {
        let mut state = State::new();
        let instruction = Instruction::new(0x61AA); // V1 = 0xAA

        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xAA, state.v[1]);
    }

    #[test]
    // 7xkk - ADD Vx, byte
    fn add_vx_byte() {
        let mut state = State::new();
        let instruction = Instruction::new(0x71AA); // V1 += 0xAA

        state.v[1] = 1;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xAB, state.v[1]);
    }

    #[test]
    // 8xy0 - LD Vx, Vy
    fn ld_vx_vy() {
        let mut state = State::new();
        let instruction = Instruction::new(0x8120); // V1 = V2

        state.v[2] = 0xAA;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xAA, state.v[1]);
    }

    #[test]
    // 8xy1 - OR Vx, Vy
    fn or_vx_vy() {
        let mut state = State::new();
        let instruction = Instruction::new(0x8121); // V1 |= V2

        state.v[1] = 0x1F;
        state.v[2] = 0xF0;
        let expected = state.v[1] | state.v[2];
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(expected, state.v[1]);
    }

    #[test]
    // 8xy2 - AND Vx, Vy
    fn and_vx_vy() {
        let mut state = State::new();
        let instruction = Instruction::new(0x8122); // V1 &= V2

        state.v[1] = 0x1F;
        state.v[2] = 0xF0;
        let expected = state.v[1] & state.v[2];
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(expected, state.v[1]);
    }

    #[test]
    // 8xy3 - XOR Vx, Vy
    fn xor_vx_vy() {
        let mut state = State::new();
        let instruction = Instruction::new(0x8123); // V1 ^= V2

        state.v[1] = 0x1F;
        state.v[2] = 0xF0;
        let expected = state.v[1] ^ state.v[2];
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(expected, state.v[1]);
    }

    #[test]
    // 8xy4 - ADD Vx, Vy
    fn add_vx_vy() {
        let mut state = State::new();
        let instruction = Instruction::new(0x8124); // V1 += V2

        // with overflow
        state.v[1] = 0xFF;
        state.v[2] = 0xF0;
        let expected = ((state.v[1] as u16 + state.v[2] as u16) % 0xFF) as u8;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(expected, state.v[1]);
        assert_eq!(1, state.v[15]);

        // without overlflow
        state.v[1] = 0x0A;
        state.v[2] = 0xA0;
        let expected = state.v[1] + state.v[2];
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(expected, state.v[1]);
        assert_eq!(0, state.v[15]);
    }

    #[test]
    // 8xy5 - SUB Vx, Vy
    fn sub_vx_vy() {
        let mut state = State::new();
        let instruction = Instruction::new(0x8125); // V1 -= V2

        // with underflow
        state.v[1] = 0xF0;
        state.v[2] = 0xFF;
        let expected = (state.v[1] as i8 - state.v[2] as i8) as u8;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(expected, state.v[1]);
        assert_eq!(0, state.v[15]);

        // without underflow
        state.v[1] = 0xFF;
        state.v[2] = 0xF0;
        let expected = state.v[1] - state.v[2];
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(expected, state.v[1]);
        assert_eq!(1, state.v[15]);
    }

    #[test]
    // 8xy6 - SHR Vx {, Vy}
    fn shr_vx_vy() {
        let mut state = State::new();
        let instruction = Instruction::new(0x8106); // V1 >> 1

        state.v[1] = 0x01;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(1, state.v[15]);
        assert_eq!(0, state.v[1]);
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0, state.v[15]);
    }

    #[test]
    // 8xy7 - SUBN Vx, Vy
    // Vx = Vy - Vx
    fn subn_vx_vy() {
        let mut state = State::new();
        let instruction = Instruction::new(0x8127); // V2 -= V1

        // with underflow
        state.v[1] = 0xFF;
        state.v[2] = 0xF0;
        let expected = (state.v[2] as i8 - state.v[1] as i8) as u8;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(expected, state.v[1]);
        assert_eq!(0, state.v[15]);

        // without underflow
        state.v[1] = 0xF0;
        state.v[2] = 0xFF;
        let expected = state.v[2] - state.v[1];
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(expected, state.v[1]);
        assert_eq!(1, state.v[15]);
    }

    #[test]
    // 8xyE - SHL Vx {, Vy}
    fn shl_vx_vy() {
        let mut state = State::new();
        let instruction = Instruction::new(0x810E); // V1 << 1

        state.v[1] = 0x80;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(1, state.v[15]);
        assert_eq!(0, state.v[1]);
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0, state.v[15]);
    }

    #[test]
    // 9xy0 - SNE Vx, Vy
    fn sne_vx_vy() {
        let mut state = State::new();
        let instruction = Instruction::new(0x9010); // V0 != V1

        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x200, state.pc);

        state.v[0] = 0xAA;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x202, state.pc);
    }

    #[test]
    // Annn - LD I, addr
    fn ld_i_addr() {
        let mut state = State::new();
        let instruction = Instruction::new(0xAAAA); // Addr = 0xAAA

        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xAAA, state.i);
    }

    #[test]
    // Bnnn - JP V0, addr
    fn jp_v0_addr() {
        let mut state = State::new();
        let instruction = Instruction::new(0xBAA0); // Addr = 0xAA0

        state.v[0] = 0x0A;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xAAA, state.pc);
    }

    // Cxkk - RND Vx, byte
    // Not testable ATM as it sets Vx to a random number

    // Dxyn - DRW Vx, Vy, nibble
    #[test]
    fn drw_vx_vy_nibble() {
        let mut state = State::new();
        let instruction = Instruction::new(0xD125); // V1, V2, 5 bytes high

        // i = 0 which points to the beginning of the character map
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(false, state.display.is_clear());
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(1, state.v[15]);
        assert_eq!(true, state.display.is_clear());
    }

    #[test]
    // Ex9E - SKP Vx
    fn skp_vx() {
        let mut state = State::new();
        let instruction = Instruction::new(0xE19E); // V1

        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x200, state.pc);
        state.v[1] = 2;
        state.keypad[2] = true;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x202, state.pc);
    }

    #[test]
    // ExA1 - SKNP Vx
    fn sknp_vx() {
        let mut state = State::new();
        let instruction = Instruction::new(0xE1A1); // V1

        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x202, state.pc);
        state.v[1] = 2;
        state.keypad[2] = true;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0x202, state.pc);
    }

    #[test]
    // Fx07 - LD Vx, DT
    fn ld_vx_dt() {
        let mut state = State::new();
        let instruction = Instruction::new(0xF107); // V1

        state.dt = 0xA;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xA, state.v[1]);
    }

    #[test]
    // Fx0A - LD Vx, K
    fn ld_vx_k() {
        let mut state = State::new();
        let instruction = Instruction::new(0xF10A); // V1

        state.keypad[0xA] = true;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(state.v[1], 0xA);
    }

    #[test]
    // Fx15 - LD DT, Vx
    fn ld_dt_vx() {
        let mut state = State::new();
        let instruction = Instruction::new(0xF115); // V1

        state.v[1] = 0xA;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xA, state.dt);
    }

    #[test]
    // Fx18 - LD ST, Vx
    fn ld_st_vx() {
        let mut state = State::new();
        let instruction = Instruction::new(0xF118); // V1

        state.v[1] = 0xA;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(0xA, state.st);
    }

    #[test]
    // Fx1E - ADD I, Vx
    fn add_i_vx() {
        let mut state = State::new();
        let instruction = Instruction::new(0xF11E); // V1

        state.v[1] = 1;
        state.i = 2;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(3, state.i);
    }

    #[test]
    // Fx29 - LD F, Vx
    fn ld_f_vx() {
        let mut state = State::new();
        let instruction = Instruction::new(0xF129); // V1

        state.v[1] = 1;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(state.v[1] * 5, state.i as u8); // Sprites are 8 x 5
    }

    #[test]
    // Fx33 - LD B, Vx
    fn ld_b_vx() {
        let mut state = State::new();
        let instruction = Instruction::new(0xF133); // V1

        state.v[1] = 128;
        state.i = 0x256;
        assert_eq!(true, instruction.function(&mut state));
        assert_eq!(1, state.ram[state.i as usize]);
        assert_eq!(2, state.ram[(state.i + 1) as usize]);
        assert_eq!(8, state.ram[(state.i + 2) as usize]);
    }

    #[test]
    // Fx55 - LD [I], Vx
    fn ld_i_vx() {
        let mut state = State::new();
        let instruction = Instruction::new(0xFA55); // FA

        state.i = 0x256;
        for i in 0..0xA {
            state.v[i] = i as u8;
        }

        assert_eq!(true, instruction.function(&mut state));
        for i in 0..0xA {
            assert_eq!(i as u8, state.ram[(state.i + i) as usize]);
        }
    }

    #[test]
    // Fx65 - LD Vx, [I]
    fn ld_vx_i() {
        let mut state = State::new();
        let instruction = Instruction::new(0xFA65); // FA

        state.i = 0x256;
        for i in 0..0xA {
            state.ram[(state.i + i) as usize] = i as u8;
        }

        assert_eq!(true, instruction.function(&mut state));
        for i in 0..0xA {
            assert_eq!(i as u8, state.v[i]);
        }
    }
}
