mod constants;
mod character;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::prelude::*;
use crate::character::Character;



fn window_conf() -> Conf {
    Conf {
        window_title: "Fluffy Fiesta".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("src/assets");

    let idle_texture = load_texture("character/Idle.png").await.unwrap();
    let walk_texture = load_texture("character/Walk.png").await.unwrap();
    idle_texture.set_filter(FilterMode::Nearest);
    walk_texture.set_filter(FilterMode::Nearest);

    let mut character = Character::new(&idle_texture, &walk_texture);

    loop {
        clear_background(BLACK);
        character.handle_keys();
        character.update();
        character.draw();

        next_frame().await;
    }
}
