mod chip8;
mod display;
mod keyboard;

use std::{fs::File, io::Read};
use chip8::Chip8;
use keyboard::KEYS;
use macroquad::prelude::*;

const CELL_SIZE: i32 = 20;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Chip 8 Emulator"),
        window_width: 64 * CELL_SIZE,
        window_height: 32 * CELL_SIZE,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut chip8 = Chip8::new();

    let mut game_file = File::open("./roms/VERS").unwrap();
    let mut rom: Vec<u8> = Vec::new();

    game_file.read_to_end(&mut rom).expect("Error reading file");

    chip8.load_rom(&rom);

    clear_background(BLACK);

    loop {

        let mut keys = [false; 16];

        for i in 0..KEYS.len() {
            keys[i] = is_key_down(KEYS[i])
        }

        chip8.keyboard.set_keys(keys);
        
        chip8.run_instruction();
        let display = chip8.get_display();

        clear_background(BLACK);
    
        for y in 0..32 {
            for x in 0..64 {
                if display[y][x] != 0 {
                    draw_rectangle((x as i32 * CELL_SIZE) as f32, (y as i32 * CELL_SIZE) as f32, CELL_SIZE as f32, CELL_SIZE as f32, WHITE);
                }
            }
        }

        next_frame().await;
    }
}
