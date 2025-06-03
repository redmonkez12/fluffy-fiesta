mod animate;
mod arrow;
mod character;
mod constants;
mod enemy;
mod map;
mod world_camera;

use crate::character::Character;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::enemy::Enemy;
use crate::map::create_map;
use crate::world_camera::WorldCamera;
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

    let idle_texture1 = load_texture("character/idle/tile000.png").await.unwrap();
    let idle_texture2 = load_texture("character/idle/tile001.png").await.unwrap();
    let idle_texture3 = load_texture("character/idle/tile002.png").await.unwrap();
    let idle_texture4 = load_texture("character/idle/tile003.png").await.unwrap();

    let walk_texture1 = load_texture("character/walk/tile000.png").await.unwrap();
    let walk_texture2 = load_texture("character/walk/tile001.png").await.unwrap();
    let walk_texture3 = load_texture("character/walk/tile002.png").await.unwrap();
    let walk_texture4 = load_texture("character/walk/tile003.png").await.unwrap();
    let walk_texture5 = load_texture("character/walk/tile004.png").await.unwrap();
    let walk_texture6 = load_texture("character/walk/tile005.png").await.unwrap();

    let jump_texture1 = load_texture("character/jump/tile000.png").await.unwrap();
    let jump_texture2 = load_texture("character/jump/tile001.png").await.unwrap();
    let jump_texture3 = load_texture("character/jump/tile002.png").await.unwrap();
    let jump_texture4 = load_texture("character/jump/tile003.png").await.unwrap();
    let jump_texture5 = load_texture("character/jump/tile004.png").await.unwrap();
    let jump_texture6 = load_texture("character/jump/tile005.png").await.unwrap();
    let jump_texture7 = load_texture("character/jump/tile006.png").await.unwrap();
    let jump_texture8 = load_texture("character/jump/tile007.png").await.unwrap();

    let walk_texture = load_texture("character/Walk.png").await.unwrap();
    let jump_texture = load_texture("character/Jump.png").await.unwrap();
    let attack_texture1 = load_texture("character/attack/tile000.png").await.unwrap();
    let attack_texture2 = load_texture("character/attack/tile001.png").await.unwrap();
    let attack_texture3 = load_texture("character/attack/tile002.png").await.unwrap();
    let attack_texture4 = load_texture("character/attack/tile003.png").await.unwrap();
    let attack_texture5 = load_texture("character/attack/tile004.png").await.unwrap();
    let attack_texture6 = load_texture("character/attack/tile005.png").await.unwrap();
    let bow_texture1 = load_texture("bow/tile000.png").await.unwrap();
    let bow_texture2 = load_texture("bow/tile001.png").await.unwrap();
    let bow_texture3 = load_texture("bow/tile002.png").await.unwrap();
    let bow_texture4 = load_texture("bow/tile003.png").await.unwrap();
    let bow_texture5 = load_texture("bow/tile004.png").await.unwrap();
    let bow_texture6 = load_texture("bow/tile005.png").await.unwrap();
    let grass_texture = load_texture("map/tiles/Tile_51.png").await.unwrap();
    let arrow_texture = load_texture("arrow.png").await.unwrap();

    let enemy_fly_texture0 = load_texture("enemy/fly/tile000.png").await.unwrap();
    let enemy_fly_texture1 = load_texture("enemy/fly/tile001.png").await.unwrap();
    let enemy_fly_texture2 = load_texture("enemy/fly/tile002.png").await.unwrap();
    let enemy_fly_texture3 = load_texture("enemy/fly/tile003.png").await.unwrap();
    let enemy_fly_texture4 = load_texture("enemy/fly/tile004.png").await.unwrap();
    let enemy_fly_texture5 = load_texture("enemy/fly/tile005.png").await.unwrap();
    let enemy_fly_texture6 = load_texture("enemy/fly/tile006.png").await.unwrap();
    let enemy_fly_texture7 = load_texture("enemy/fly/tile007.png").await.unwrap();

    let enemy_idle_texture1 = load_texture("enemy/idle/tile000.png").await.unwrap();
    let enemy_idle_texture2 = load_texture("enemy/idle/tile001.png").await.unwrap();
    let enemy_idle_texture3 = load_texture("enemy/idle/tile002.png").await.unwrap();
    let enemy_idle_texture4 = load_texture("enemy/idle/tile003.png").await.unwrap();
    let enemy_idle_texture5 = load_texture("enemy/idle/tile004.png").await.unwrap();
    let enemy_idle_texture6 = load_texture("enemy/idle/tile005.png").await.unwrap();
    let enemy_idle_texture7 = load_texture("enemy/idle/tile006.png").await.unwrap();
    let enemy_idle_texture8 = load_texture("enemy/idle/tile007.png").await.unwrap();

    let enemy_hit_texture1 = load_texture("enemy/hit/tile000.png").await.unwrap();
    let enemy_hit_texture2 = load_texture("enemy/hit/tile001.png").await.unwrap();
    let enemy_hit_texture3 = load_texture("enemy/hit/tile002.png").await.unwrap();
    let enemy_hit_texture4 = load_texture("enemy/hit/tile003.png").await.unwrap();

    let enemy_die_texture1 = load_texture("enemy/die/tile000.png").await.unwrap();
    let enemy_die_texture2 = load_texture("enemy/die/tile001.png").await.unwrap();
    let enemy_die_texture3 = load_texture("enemy/die/tile002.png").await.unwrap();
    let enemy_die_texture4 = load_texture("enemy/die/tile003.png").await.unwrap();
    let enemy_die_texture5 = load_texture("enemy/die/tile004.png").await.unwrap();
    let enemy_die_texture6 = load_texture("enemy/die/tile005.png").await.unwrap();
    let enemy_die_texture7 = load_texture("enemy/die/tile006.png").await.unwrap();
    let enemy_die_texture8 = load_texture("enemy/die/tile007.png").await.unwrap();
    let enemy_die_texture9 = load_texture("enemy/die/tile008.png").await.unwrap();
    let enemy_die_texture10 = load_texture("enemy/die/tile009.png").await.unwrap();
    let enemy_die_texture11 = load_texture("enemy/die/tile010.png").await.unwrap();
    let enemy_die_texture12 = load_texture("enemy/die/tile011.png").await.unwrap();
    let enemy_die_texture13 = load_texture("enemy/die/tile012.png").await.unwrap();
    let enemy_die_texture14 = load_texture("enemy/die/tile013.png").await.unwrap();
    let enemy_die_texture15 = load_texture("enemy/die/tile014.png").await.unwrap();

    idle_texture1.set_filter(FilterMode::Nearest);
    idle_texture2.set_filter(FilterMode::Nearest);
    idle_texture3.set_filter(FilterMode::Nearest);
    idle_texture4.set_filter(FilterMode::Nearest);
    walk_texture.set_filter(FilterMode::Nearest);
    jump_texture.set_filter(FilterMode::Nearest);
    grass_texture.set_filter(FilterMode::Nearest);
    let idle_textures = vec![idle_texture1, idle_texture2, idle_texture3, idle_texture4];
    let walk_textures = vec![
        walk_texture1,
        walk_texture2,
        walk_texture3,
        walk_texture4,
        walk_texture5,
        walk_texture6,
    ];
    let jump_textures = vec![
        jump_texture1,
        jump_texture2,
        jump_texture3,
        jump_texture4,
        jump_texture5,
        jump_texture6,
        jump_texture7,
        jump_texture8,
    ];
    let attack_textures = vec![
        attack_texture1,
        attack_texture2,
        attack_texture3,
        attack_texture4,
        attack_texture5,
        attack_texture6,
    ];
    let bow_textures = vec![
        bow_texture1,
        bow_texture2,
        bow_texture3,
        bow_texture4,
        bow_texture5,
        bow_texture6,
    ];
    let enemy_idle_textures = vec![
        enemy_fly_texture0,
        enemy_fly_texture1,
        enemy_fly_texture2,
        enemy_fly_texture3,
        enemy_fly_texture4,
        enemy_fly_texture5,
        enemy_fly_texture6,
        enemy_fly_texture7,
    ];
    let enemy_fly_textures = vec![
        enemy_idle_texture1,
        enemy_idle_texture2,
        enemy_idle_texture3,
        enemy_idle_texture4,
        enemy_idle_texture5,
        enemy_idle_texture6,
        enemy_idle_texture7,
        enemy_idle_texture8,
    ];
    let enemy_hit_textures = vec![
        enemy_hit_texture1,
        enemy_hit_texture2,
        enemy_hit_texture3,
        enemy_hit_texture4,
    ];
    let enemy_die_textures = vec![
        enemy_die_texture1,
        enemy_die_texture2,
        enemy_die_texture3,
        enemy_die_texture4,
        enemy_die_texture5,
        enemy_die_texture6,
        enemy_die_texture7,
        enemy_die_texture8,
        enemy_die_texture9,
        enemy_die_texture10,
        enemy_die_texture11,
        enemy_die_texture12,
        enemy_die_texture13,
        enemy_die_texture14,
        enemy_die_texture15,
    ];

    let tilemap = create_map(&grass_texture);

    let tile_size = grass_texture.height();
    let map_height = tilemap.len();
    let y_offset = SCREEN_HEIGHT - (map_height as f32 * tile_size);
    let mut character = Character::new(
        &idle_textures,
        &walk_textures,
        &jump_textures,
        &attack_textures,
        &arrow_texture,
        &bow_textures,
    );

    let enemy = Enemy::new(
        &enemy_idle_textures,
        &enemy_fly_textures,
        &enemy_hit_textures,
        &enemy_die_textures,
    );

   let mut enemies = vec![enemy];

    let map_width = tilemap[0].len() as f32 * tile_size;
    let map_height = tilemap.len() as f32 * tile_size;
    let mut world_camera = WorldCamera::new(map_width, map_height);

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        let character_pos = Vec2::new(character.rect.center().x, character.rect.center().y);
        world_camera.follow_target(character_pos, dt);
        let camera_offset = Vec2::new(world_camera.position.x, world_camera.position.y);
        character.set_camera_offset(camera_offset);

        set_camera(&world_camera.get_camera2d());

        for arrow in character.arrows.iter_mut() {
            arrow.update(dt);
            arrow.draw();
        }
        
        for enemy in enemies.iter_mut() {
            enemy.update();
            enemy.draw();
        }

        character.handle_keys();
        character.update();
        draw_tilemap(&tilemap, tile_size, y_offset);

        check_tilemap_collision(&mut character, &mut enemies, &tilemap, tile_size, y_offset);
   
        character.draw();

        set_default_camera();

        next_frame().await;
    }
}

fn check_tilemap_collision(
    character: &mut Character,
    enemies: &mut Vec<Enemy>,
    tilemap: &Vec<Vec<Option<Texture2D>>>,
    tile_size: f32,
    y_offset: f32,
) {
    let collision_rect = Rect::new(
        character.rect.x,
        character.rect.y,
        character.rect.w,
        character.rect.h,
    );

    character.arrows.retain(|arrow| !arrow.should_remove);
    enemies.retain(|enemy| !enemy.is_dead());

    for (y, row) in tilemap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Some(_tile) = tile {
                let tile_x = x as f32 * tile_size;
                let tile_y = y_offset + y as f32 * tile_size;
                let tile_rect = Rect::new(tile_x, tile_y, tile_size, tile_size);

                for arrow in character.arrows.iter_mut() {
                    if !arrow.stuck {
                        for enemy in enemies.iter_mut() {
                            if enemy.can_be_hit() && enemy.rect.overlaps(&arrow.rect) {
                                enemy.hit();
                                arrow.stuck_angle = arrow.velocity.y.atan2(arrow.velocity.x);
                                arrow.velocity = vec2(0.0, 0.0);
                                arrow.stuck = true;
                                arrow.stuck_timer = 2.9;
                            }
                        }
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
                        character.rect.x = tile_rect.left() - character.rect.w;
                    } else if min_value == from_right {
                        character.rect.x = tile_rect.right();
                    } else if min_value == from_top {
                        character.rect.y = tile_rect.top() - character.rect.h;
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
