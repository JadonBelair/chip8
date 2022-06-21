pub struct Display {
    screen: [[u8; 64]; 32],

    pub wrap: bool
}

impl Display {
    pub fn new() -> Self {
        Self {
            screen: [[0; 64]; 32],
            wrap: true
        }
    }

    pub fn clear(&mut self) {
        self.screen = [[0; 64]; 32];
    }

    pub fn draw_byte(&mut self, byte: u8, x: usize, y: usize) -> bool {
        let mut col = false;

        for i in 0..8 {
            let mut coord_x: usize = x as usize + i;
            let mut coord_y: usize = y as usize;

            if self.wrap {
                coord_x %= 64;
                coord_y %= 32;
            }

            if coord_x < 64 && coord_y < 32 && (byte & (0x80 >> i)) != 0 { 
                if self.screen[coord_y][coord_x] == 1 {
                    col = true;
                }

                self.screen[coord_y][coord_x] ^= 1;
            }
        }

        col
    }

    pub fn get_screen(&self) -> [[u8; 64]; 32] {
        self.screen
    }

}