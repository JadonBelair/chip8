mod chip8;
mod display;
mod keyboard;

use std::{fs::{read_dir, File}, io::Read};
use chip8::Chip8;
use keyboard::KEYS;
use macroquad::{ui::{root_ui, widgets}, prelude::*};

const CELL_SIZE: i32 = 20;

const Y_OFFSET: f32 = 30.;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Chip 8 Emulator"),
        window_width: 64 * CELL_SIZE,
        window_height: (32 * CELL_SIZE) + Y_OFFSET as i32,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut chip8 = Chip8::new();

    // loads the rom with a single instruction to jump to itself
    let mut rom: Vec<u8> = vec![0x12, 0x00];

    // gets all the games from the roms directory
    let games: Vec<String> = if let Ok(dir) = read_dir(".\\rosms"){
        dir.map(|x| x.unwrap().path().to_str().unwrap().to_owned()).collect()
    } else {
        Vec::new()
    };

    let mut show_games = false;

    chip8.load_rom(&rom);

    loop {

        // sets every key that is currently down
        let mut keys = [false; 16];
        for i in 0..KEYS.len() {
            keys[i] = is_key_down(KEYS[i])
        }
        chip8.keyboard.set_keys(keys);
        
        // runs a single instruction from the ram
        chip8.run_instruction();

        // grabs the current state of the display for drawing
        let display = chip8.get_display();

        clear_background(BLACK);
        
        // draws the display
        for y in 0..32 {
            for x in 0..64 {
                // only draws white pixels
                if display[y][x] != 0 {
                    draw_rectangle((x as i32 * CELL_SIZE) as f32, (y as i32 * CELL_SIZE) as f32 + Y_OFFSET, CELL_SIZE as f32, CELL_SIZE as f32, WHITE);
                }
            }
        }

        // draws the bar for the UI
        draw_rectangle(0., 0., screen_width(), Y_OFFSET, GRAY);

        // creates the UI buttons
        let roms_button = widgets::Button::new("Load Rom").position(vec2(5., 5.)).size(vec2(80., 20.));
        let wrap_button = widgets::Button::new(format!("{}able Screen Wrapping", if chip8.display.wrap {"Dis"} else {"En"}))
                        .position(vec2(90.,  5.)).size(vec2(200., 20.));

        // checks if the user clicked the button to load a rom
        // and inverts the boolean for showing the list of games
        if roms_button.ui(&mut *root_ui()) {
            show_games = !show_games;
        }

        // checks if the user wants screen wrapping enabled
        if wrap_button.ui(&mut *root_ui()) {
            chip8.display.wrap = !chip8.display.wrap;
        }

        // draw a list of all available roms if the user requested it
        if show_games {
            let window = widgets::Window::new(1, vec2(5., 25.), vec2(300.,
                         if games.len() > 0 {500.} else {50.})).movable(false);

            window.ui(&mut *root_ui(), |ui| {
                // only draws the game titles if there are any games to play
                if games.len() > 0 {
                    for i in 0..games.len() {
                        if ui.button(vec2(0., 20. * i as f32), *&games[i].as_str()) {
                            rom = Vec::new();

                            let mut game_file = File::open(&games[i]).unwrap();
                            game_file.read_to_end(&mut rom).unwrap();

                            chip8.reset();

                            chip8.load_rom(&rom);

                            // hides the game menu after selecting a rom
                            show_games = false;
                        }
                    }
                // if there are no games, tell the user to add some
                } else {
                    ui.label(vec2(20., 10.), "Please add games to the roms folder.");
                }
            });
        }

        next_frame().await;
    }
}
