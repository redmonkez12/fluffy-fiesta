mod animate;
mod arrow;
mod character;
mod constants;
mod map;
mod enemy;
mod world_camera;

use crate::character::Character;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::prelude::*;
// use crate::enemy::Enemy;
use crate::map::create_map;
use crate::world_camera::WorldCamera;

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

    let idle_texture1 = load_texture("character/idle/tile000.png").await.unwrap();
    let idle_texture2 = load_texture("character/idle/tile001.png").await.unwrap();
    let idle_texture3 = load_texture("character/idle/tile002.png").await.unwrap();
    let idle_texture4 = load_texture("character/idle/tile003.png").await.unwrap();
    let walk_texture = load_texture("character/Walk.png").await.unwrap();
    let jump_texture = load_texture("character/Jump.png").await.unwrap();
    let attack_texture = load_texture("character/Attack.png").await.unwrap();
    let bow_texture = load_texture("character/3.png").await.unwrap();
    let grass_texture = load_texture("map/tiles/Tile_51.png").await.unwrap();
    let arrow_texture = load_texture("arrow.png").await.unwrap();
    let enemy_idle_state = load_texture("enemy_1/Enemy3No-Move-Idle.png").await.unwrap();
    let enemy_fly_state = load_texture("enemy_1/Enemy3No-Move-Fly.png").await.unwrap();
    let enemy_hit_state = load_texture("enemy_1/Enemy3No-Move-Hit.png").await.unwrap();
    let enemy_die_state = load_texture("enemy_1/Enemy3No-Move-Die.png").await.unwrap();

    idle_texture1.set_filter(FilterMode::Nearest);
    idle_texture2.set_filter(FilterMode::Nearest);
    idle_texture3.set_filter(FilterMode::Nearest);
    idle_texture4.set_filter(FilterMode::Nearest);
    walk_texture.set_filter(FilterMode::Nearest);
    jump_texture.set_filter(FilterMode::Nearest);
    grass_texture.set_filter(FilterMode::Nearest);
    let idle_textures = vec![idle_texture1, idle_texture2, idle_texture3, idle_texture4];
    
    let tilemap = create_map(&grass_texture);

    let tile_size = grass_texture.height();
    let map_height = tilemap.len();
    let y_offset = SCREEN_HEIGHT - (map_height as f32 * tile_size);
    let mut character = Character::new(&idle_textures);

    // let mut enemy = Enemy::new(&enemy_idle_state, &enemy_fly_state, &enemy_hit_state, &enemy_die_state);

    let map_width = tilemap[0].len() as f32 * tile_size;
    let map_height = tilemap.len() as f32 * tile_size;
    let mut world_camera = WorldCamera::new(map_width, map_height);

    loop {
        clear_background(BLACK);

        let dt = get_frame_time() ;

        let character_pos = Vec2::new(character.rect.center().x, character.rect.center().y);
        world_camera.follow_target(character_pos, dt);

        set_camera(&world_camera.get_camera2d());

        for arrow in character.arrows.iter_mut() {
            arrow.update(dt);
            arrow.draw();
        }

        // enemy.update();

        character.handle_keys();
        character.update();
        draw_tilemap(&tilemap, tile_size, y_offset);

        check_tilemap_collision(&mut character, &tilemap, tile_size, y_offset);
        // enemy.draw();
        character.draw();

        set_default_camera();

        next_frame().await;
    }
}

fn check_tilemap_collision(
    character: &mut Character,
    // enemy: &mut Enemy,
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

    character.arrows.retain(|arrow| !arrow.should_remove);

    for (y, row) in tilemap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Some(_tile) = tile {
                let tile_x = x as f32 * tile_size;
                let tile_y = y_offset + y as f32 * tile_size;
                let tile_rect = Rect::new(tile_x, tile_y, tile_size, tile_size);

                for arrow in character.arrows.iter_mut() {
                    if !arrow.stuck {
                        // if enemy.can_be_hit() && enemy.rect.overlaps(&arrow.rect) {
                        //     enemy.hit();
                        //     arrow.stuck_angle = arrow.velocity.y.atan2(arrow.velocity.x);
                        //     arrow.velocity = vec2(0.0, 0.0);
                        //     arrow.stuck = true;
                        //     arrow.stuck_timer = 2.9;
                        // }
                        if arrow.check_collision_and_embed(&tile_rect, 15.0) {
                            arrow.stuck_angle = arrow.velocity.y.atan2(arrow.velocity.x);
                            arrow.velocity = vec2(0.0, 0.0);
                            arrow.stuck = true;
                            arrow.stuck_timer = 0.0;
                        }
                    }
                }
                
                if collision_rect.overlaps(&tile_rect) {
                    let from_left = collision_rect.right() - tile_rect.left();
                    let from_right = tile_rect.right() - collision_rect.left();
                    let from_top = collision_rect.bottom() - tile_rect.top();
                    let from_bottom = tile_rect.bottom() - collision_rect.top();

                    let min_value = from_left.min(from_right).min(from_top).min(from_bottom);

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
                        character.rect.y = tile_rect.bottom();
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
