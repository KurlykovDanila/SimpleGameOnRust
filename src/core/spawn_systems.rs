use std::f32::consts::PI;

use hecs::{Entity, World};
use macroquad::prelude::*;

use crate::core::components::*;

pub fn spawn_ship(world: &mut World) -> Entity {
    world.spawn((
        Health(10),
        CurrentHealth(10),
        Damage(1),
        Position(Vec2::new(screen_width() / 2., screen_height() / 2.)),
        Rotation(0.),
        Gun,
        Direction(Vec2::new(0., 1.)),
        Speed(0.),
        Ship,
        LastShootTime(0.),
        ReloadTime(0.5),
        Option::<FlyText>::None,
    ))
}

pub fn spawn_asteroids(world: &mut World, n: usize, lvl: f32) {
    let to_spawn = (0..n).map(|_| {
        let pos = Position(Vec2::new(
            rand::gen_range(0., screen_width()),
            rand::gen_range(0., screen_height()),
        ));
        let d = Direction(Vec2::new(rand::gen_range(0., 1.), rand::gen_range(0., 1.)).normalize());
        let speed = Speed(rand::gen_range(10., 40.) * lvl);
        let hp = Health(rand::gen_range(2, 10));
        let c_hp = CurrentHealth(hp.0);
        let dmg = Damage(rand::gen_range(2, 7));
        let obj = Asteroid;
        let rot = Rotation(rand::gen_range(2., 7.));
        let size = Size(rand::gen_range(7., 15.));
        let fl_text = Option::<FlyText>::None;

        (pos, speed, d, hp, dmg, obj, rot, size, c_hp, fl_text)
    });

    world.spawn_batch(to_spawn);
}

pub fn spawn_bullet(world: &mut World, ship: Entity, time_now: f64) {
    {
        let mut last_time_shoot = world.get::<&mut LastShootTime>(ship).unwrap();
        let reload_time = world.get::<&ReloadTime>(ship).unwrap();
        if time_now - last_time_shoot.0 < reload_time.0 {
            return;
        } else {
            last_time_shoot.0 = time_now;
        }
    }
    let bullet = {
        let _ship_pos = world.get::<&Position>(ship).unwrap();
        let _ship_speed = world.get::<&Speed>(ship).unwrap();
        let _ship_dir = world.get::<&Direction>(ship).unwrap();
        let _ship_dmg = world.get::<&Damage>(ship).unwrap();
        let pos = Position(_ship_pos.0.clone());
        let speed = Speed(_ship_speed.0.max(0.) + 150.);
        let dir = Direction(_ship_dir.0.normalize());
        let damage = Damage(_ship_dmg.0);
        let size = Size(5.);
        (pos, dir, speed, damage, Bullet, size)
    };
    world.spawn(bullet);
}
