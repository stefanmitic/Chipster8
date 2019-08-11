use std::fmt;

pub struct Display {
    pub data: [[u8; 64]; 32], // 64 x 32 pixels, 1b == 1pix
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.iter() {
            for pixel in row.iter() {
                let _ = write!(f, "{}", pixel);
            }
            let _ = write!(f, "\n");
        }
        Ok(())
    }
}

impl Display {
    pub fn new() -> Display {
        Display {
            data: [[0u8; 64]; 32],
        }
    }

    // Wraps sprites if x and y are bigger than 63 or 31
    // Returns true if there was a collision (some pixel was reset)
    pub fn display_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut collision: bool = false;
        for (row, byte) in sprite.iter().enumerate() {
            let y = ((y + row as u8) % 32) as usize;
            for i in 0..7 {
                let x = ((x + i) % 64) as usize;
                self.data[y][x] ^= if (0b10000000 >> i & byte) > 0 { 1 } else { collision = true; 0 };
            }
        }
        collision
    }

    // Resets the display to all 0
    pub fn reset(&mut self) {
        for row in self.data.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = 0;
            }
        }
    }

    // Check if the display is clean, mostly used in tests
    pub fn is_clear(&self) -> bool {
        for row in self.data.iter() {
            for pixel in row.iter() {
                if *pixel != 0u8 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_sprite_test() {
        let mut display: Display = Display::new();
        let sprite = [0xF0, 0x90, 0x90, 0x90, 0xF0];

        display.display_sprite(0, 0, &sprite);

        display.display_sprite(10, 10, &sprite);

        display.display_sprite(64, 32, &sprite);

        assert_eq!(false, display.is_clear());
    }

    #[test]
    fn reset_test() {
        let mut display: Display = Display::new();
        let sprite = [0xF0, 0x90, 0x90, 0x90, 0xF0];

        display.display_sprite(0, 0, &sprite);

        assert_eq!(false, display.is_clear());

        display.reset();

        assert_eq!(true, display.is_clear());
    }
}
