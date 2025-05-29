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
    let grass = load_texture("map/tiles/Tile_51.png").await.unwrap();

    idle_texture.set_filter(FilterMode::Nearest);
    walk_texture.set_filter(FilterMode::Nearest);
    jump_texture.set_filter(FilterMode::Nearest);
    grass.set_filter(FilterMode::Nearest);

    let tilemap: Vec<Vec<Option<Texture2D>>> = vec![
        vec![None; 25],
        vec![None; 25],
        vec![None; 25],
        {
            let mut row = vec![None; 25];
            row[10] = Some(grass.clone());
            row[11] = Some(grass.clone());
            row[12] = Some(grass.clone());
            row
        },
        vec![None; 25],
        {
            let mut row = vec![None; 25];
            row[3] = Some(grass.clone());
            row[4] = Some(grass.clone());
            row[6] = Some(grass.clone());
            row[7] = Some(grass.clone());
            row
        },
        vec![None; 25],
        {
            let mut row = vec![None; 25];
            for i in 8..15 {
                if i != 12 {
                    row[i] = Some(grass.clone());
                }
            }
            row
        },
        {
            let mut row = vec![None; 25];
            row[2] = Some(grass.clone());
            row[17] = Some(grass.clone());
            row[22] = Some(grass.clone());
            row
        },
        vec![Some(grass.clone()); 25],
    ];


    let tile_size = grass.height();
    let map_height = tilemap.len();
    let y_offset = SCREEN_HEIGHT - (map_height as f32 * tile_size);
    let mut character = Character::new(&idle_texture, &walk_texture, &jump_texture);

    loop {
        clear_background(BLACK);

        character.handle_keys();
        check_tilemap_collision(&mut character, &tilemap, tile_size, y_offset);
        character.update();
        draw_tilemap(&tilemap, tile_size, y_offset);
        character.draw();
        next_frame().await;
    }
}

fn check_tilemap_collision(
    character: &mut Character,
    tilemap: &Vec<Vec<Option<Texture2D>>>,
    tile_size: f32,
    y_offset: f32,
) {
    let collision_shrink = 4.0;
    let collision_rect = Rect::new(
        character.rect.x + collision_shrink,
        character.rect.y + collision_shrink,
        character.rect.w - (collision_shrink * 2.0),
        character.rect.h - (collision_shrink * 2.0),
    );
    
    for (y, row) in tilemap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Some(_tile) = tile {
                let tile_x = x as f32 * tile_size;
                let tile_y = y_offset + y as f32 * tile_size;
                let tile_rect = Rect::new(tile_x, tile_y, tile_size, tile_size);

                if collision_rect.overlaps(&tile_rect) {
                    let from_left = collision_rect.right() - 5.0 - tile_rect.left();
                    let from_right = tile_rect.right() - collision_rect.left() - 5.0;
                    let from_top = collision_rect.bottom() - 5.0 - tile_rect.top();
                    let from_bottom = tile_rect.bottom() - collision_rect.top() + 5.0;

                    let min_value = from_left
                        .min(from_right)
                        .min(from_top)
                        .min(from_bottom);
                    
                    if min_value == from_left {
                        character.rect.x = tile_rect.left() - character.rect.w + collision_shrink;
                    } else if min_value == from_right {
                        character.rect.x = tile_rect.right() - collision_shrink;
                    } else if min_value == from_top {
                        character.rect.y = tile_rect.top() - character.rect.h + collision_shrink;
                        character.velocity.y = 0.0;
                        character.on_ground = true;
                        character.is_jumping = false;
                    } else if min_value == from_bottom {
                        character.rect.y = tile_rect.bottom() - collision_shrink;
                        character.velocity.y = 0.0;
                    }
                }
            }
        }
    }
}

fn draw_tilemap(tilemap: &Vec<Vec<Option<Texture2D>>>, tile_size: f32, y_offset: f32) {
    for (y, row) in tilemap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Some(texture) = tile {
                draw_texture(
                    texture,
                    x as f32 * tile_size,
                    y_offset + y as f32 * tile_size,
                    WHITE,
                );
            }
        }
    }
}
