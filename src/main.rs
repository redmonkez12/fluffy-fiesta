mod animate;
mod character;
mod constants;

use crate::character::Character;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::prelude::*;

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
    let jump_texture = load_texture("character/Jump.png").await.unwrap();
    let grass = Some(load_texture("map/tiles/Tile_51.png").await.unwrap());
    idle_texture.set_filter(FilterMode::Nearest);
    walk_texture.set_filter(FilterMode::Nearest);

    let tilemap: Vec<Vec<Option<Texture2D>>> = vec![
        vec![None, None, None, None, None, None, None, None, None, None],
        vec![None, None, None, None, None, None, None, None, None, None],
        vec![None, None, None, None, None, None, None, None, None, None],
        vec![None, None, None, None, None, None, None, None, None, None],
        vec![None, None, None, None, None, None, None, None, None, None],
        vec![
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            grass.clone(),
            grass.clone(),
            None,
            None,
            None,
            None,
        ],
        vec![None, None, None, None, None, None, None, None, None, None],
        vec![
            None,
            None,
            None,
            None,
            grass.clone(),
            grass.clone(),
            None,
            None,
            None,
            None,
        ],
        vec![None, None, None, None, None, None, None, None, None, None],
        vec![
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
            grass.clone(),
        ],
    ];

    let tile_size = grass.unwrap().height();
    let map_height = tilemap.len();
    let y_offset = SCREEN_HEIGHT - (map_height as f32 * tile_size);
    let mut character = Character::new(&idle_texture, &walk_texture, &jump_texture);

    loop {
        clear_background(BLACK);

        for (y, row) in tilemap.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Some(tile) = tile {
                    let pos_x = x as f32 * tile_size;
                    let pos_y = y_offset + y as f32 * tile_size;
                    let texture_rect = Rect::new(pos_x, pos_y, tile_size, tile_size);
                    if texture_rect.overlaps(&character.rect) {
                        // let overlap_top = character_rect.bottom() - texture_rect.top();

                        println!("{}", texture_rect.top());
                        
                        if character.jump_vec.y > 0.0 {
                            character.rect.y = texture_rect.top() - character.rect.h;
                            character.jump_vec.y = 0.0;
                            character.jump_vec.x = 0.0;
                            character.on_ground = true;
                            character.is_jumping = false;
                        }
                    }

                    draw_texture_ex(
                        &tile,
                        pos_x,
                        pos_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(tile_size, tile_size)),
                            ..Default::default()
                        },
                    )
                }
            }
        }

        character.handle_keys();
        character.update();
        character.draw();

        next_frame().await;
    }
}
