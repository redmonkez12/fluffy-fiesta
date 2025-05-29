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
        vec![None; 25],
        vec![None; 25],
        {
            let mut row = vec![None; 25];
            row[7] = Some(grass.clone());
            row[8] = Some(grass.clone());
            row
        },
        vec![None; 25],
        {
            let mut row = vec![None; 25];
            row[4] = Some(grass.clone());
            row[5] = Some(grass.clone());
            row[6] = Some(grass.clone());
            row[7] = Some(grass.clone());
            row
        },
        vec![None; 25],
        vec![Some(grass.clone()); 25],
    ];

    let tile_size = grass.height();
    let map_height = tilemap.len();
    let y_offset = SCREEN_HEIGHT - (map_height as f32 * tile_size);
    let mut character = Character::new(&idle_texture, &walk_texture, &jump_texture);

    println!("Tile size: {}", tile_size);
    println!("Map height: {}", map_height);
    println!("Y offset: {}", y_offset);
    println!(
        "Ground level should be at: {}",
        y_offset + (map_height - 1) as f32 * tile_size
    );

    let ground_y = y_offset + (map_height - 1) as f32 * tile_size - character.rect.h;
    character.rect.y = ground_y;
    character.on_ground = true;
    println!("Setting character to ground at y: {}", ground_y);

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

fn check_tilemap_collision(character: &mut Character, tilemap: &Vec<Vec<Option<Texture2D>>>, tile_size: f32, y_offset: f32) {
    for (y, row) in tilemap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Some(tile) = tile {
                let pos_x = x as f32 * tile_size;
                let pos_y = y_offset + y as f32 * tile_size;
                let texture_rect = Rect::new(pos_x, pos_y, tile_size, tile_size);
                if character.rect.overlaps(&texture_rect) {
                    // Get the exact overlapping rectangle
                    if let Some(intersection) = character.rect.intersect(texture_rect) {
                        let char_center_x = character.rect.center().x;
                        let char_center_y = character.rect.center().y;
                        let tile_center_x = texture_rect.center().x;
                        let tile_center_y = texture_rect.center().y;

                        let dx = char_center_x - tile_center_x;
                        let dy = char_center_y - tile_center_y;

                        let horizontal_overlap = intersection.w;
                        let vertical_overlap = intersection.h;

                        println!(
                            "Character velocity - x: {}, y: {}",
                            character.velocity.x, character.velocity.y
                        );
                        println!(
                            "Overlap - horizontal: {}, vertical: {}",
                            horizontal_overlap, vertical_overlap
                        );
                        println!("Distance from centers - dx: {}, dy: {}", dx, dy);

                        if horizontal_overlap < vertical_overlap {
                            if dx > 0.0 {
                                println!("Collision: Character hitting tile from LEFT");
                                character.rect.x = texture_rect.right();
                            } else {
                                println!("Collision: Character hitting tile from RIGHT");
                                character.rect.x = texture_rect.left() - character.rect.w;
                            }
                            character.velocity.x = 0.0;
                        } else {
                            if dy > 0.0 && character.velocity.y > 0.0 {
                                println!("Collision: Character LANDING ON TOP");
                                character.rect.y = texture_rect.top() - character.rect.h;
                                character.velocity.y = 0.0;
                                character.on_ground = true;
                                character.is_jumping = false;
                            } else if dy < 0.0 && character.velocity.y < 0.0 {
                                println!("Collision: Character HITTING FROM BELOW");
                                character.rect.y = texture_rect.bottom();
                                character.velocity.y = 0.0;
                            }
                        }
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
