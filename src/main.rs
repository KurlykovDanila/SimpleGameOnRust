mod core;
use core::{clear_fly_text, has_asteroids};

use hecs::World;
use macroquad::prelude::*;
use macroquad::{
    color::{RED, WHITE},
    input::{get_keys_down, is_key_down, KeyCode},
    text::draw_text,
    time::{get_frame_time, get_time},
    window::{next_frame, screen_height, screen_width},
};

use crate::core::{
    collision_system::*, components::*, motion_system::*, render_system::*, spawn_systems::*,
};

#[macroquad::main("Asteroids")]
async fn main() {
    let mut asteroids_count = 4;
    let mut speed_lvl = 1.;
    let mut world = World::new();
    let mut ship = spawn_ship(&mut world);
    spawn_asteroids(&mut world, asteroids_count, speed_lvl);
    let mut game_res: Option<bool> = None;
    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        if world.get::<&CurrentHealth>(ship).unwrap().0 == 0 {
            game_res = Some(false);
        }
        if !has_asteroids(&mut world) {
            game_res = Some(true);
        }
        if let Some(res) = game_res {
            if res {
                let _text_size = measure_text(
                    format!("You WIN (Level: {})!!!", asteroids_count / 4).as_str(),
                    None,
                    60. as _,
                    1.0,
                );
                speed_lvl *= 1.5;
                asteroids_count += 4;
                draw_text(
                    format!("You WIN (Level: {})!!!", asteroids_count / 4).as_str(),
                    screen_width() / 2. - _text_size.width / 2.,
                    screen_height() / 2. - _text_size.height / 2.,
                    60.,
                    WHITE,
                )
            } else {
                speed_lvl = 1.;
                asteroids_count = 4;
                let _text_size = measure_text(
                    format!("You LOSE (Record: {} level)!!!", asteroids_count / 4).as_str(),
                    None,
                    60. as _,
                    1.0,
                );
                draw_text(
                    format!("You LOSE (Record: {} level)!!!", asteroids_count / 4).as_str(),
                    screen_width() / 2. - _text_size.width / 2.,
                    screen_height() / 2. - _text_size.height / 2.,
                    60.,
                    RED,
                )
            }
            let _text_size = measure_text("To restart game press [ENTER]", None, 40. as _, 1.0);
            draw_text(
                "To continue game press [ENTER]",
                screen_width() / 2. - _text_size.width / 2.,
                screen_height() / 2. - _text_size.height / 2. + 60.,
                40.,
                WHITE,
            );
            if is_key_down(KeyCode::Enter) {
                world.clear();
                ship = spawn_ship(&mut world);
                spawn_asteroids(&mut world, asteroids_count, speed_lvl);
                game_res = None;
            }
            next_frame().await;
            continue;
        }
        let time_now = get_time();
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Space) {
            spawn_bullet(&mut world, ship, time_now);
        }
        if is_key_down(KeyCode::W) {
            world.get::<&mut Speed>(ship).unwrap().0 += 60. * delta_time;
        } else if is_key_down(KeyCode::S) {
            world.get::<&mut Speed>(ship).unwrap().0 -= 80. * delta_time;
        }

        if is_key_down(KeyCode::A) {
            world.get::<&mut Rotation>(ship).unwrap().0 -= 2. * delta_time;
        } else if is_key_down(KeyCode::D) {
            world.get::<&mut Rotation>(ship).unwrap().0 += 2. * delta_time;
        }

        clear_fly_text(&mut world, time_now);
        motion_system(&mut world, delta_time);
        ship_and_asteroids_collision(&mut world, ship);
        bullets_and_asteroids_collision(&mut world);
        ship_render(&mut world, ship, time_now);
        bullet_render(&mut world);
        asteroid_render(&mut world);

        next_frame().await;
    }
}
