pub struct Display {
    screen: [[u8; 64]; 32]
}

impl Display {
    pub fn new() -> Self {
        Self {
            screen: [[0; 64]; 32]
        }
    }

    pub fn clear(&mut self) {
        self.screen = [[0; 64]; 32];
    }

    pub fn draw_byte(&mut self, byte: u8, x: usize, y: usize) -> bool {
        let mut col = false;

        for i in 0..8 {
            let coord_x: usize = (x as usize + i) % 64;
            let coord_y: usize = (y as usize) % 32;
            if (byte & (0x80 >> i)) != 0 {
                if self.screen[coord_y][coord_x] == 1 {
                    col = true;
                }

                self.screen[coord_y][coord_x] ^= 1;
            }
        }

        return col;
    }

    pub fn get_screen(&self) -> [[u8; 64]; 32] {
        return self.screen;
    }

}