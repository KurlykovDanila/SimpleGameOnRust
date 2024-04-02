use std::f32::consts::PI;

use hecs::World;
use macroquad::prelude::*;

use crate::{core::components::*, delete_entites};

fn translate_coord(coord: &mut Position) {
    if coord.0.x < 0. {
        coord.0.x = screen_width() + coord.0.x;
    }
    if coord.0.y < 0. {
        coord.0.y = screen_height() + coord.0.y;
    }
    coord.0.x = coord.0.x % screen_width();
    coord.0.y = coord.0.y % screen_height();
}

pub fn motion_system(world: &mut World, delta_time: f32) {
    let mut deleted = Vec::new();
    for (_, (pos, dir, sp, _)) in
        world.query_mut::<(&mut Position, &Direction, &Speed, &Asteroid)>()
    {
        pos.0 += dir.0.normalize() * sp.0 * delta_time;
        translate_coord(pos);
    }
    for (bullet, (pos, dir, sp, _)) in
        world.query_mut::<(&mut Position, &Direction, &Speed, &Bullet)>()
    {
        pos.0 += dir.0.normalize() * sp.0 * delta_time;
        if pos.0.x < 0. || pos.0.y < 0. || pos.0.x > screen_width() || pos.0.y > screen_height() {
            deleted.push(bullet);
        }
    }
    delete_entites(world, deleted);
    for (_, (pos, dir, sp, _)) in
        world.query_mut::<(&mut Position, &Direction, &mut Speed, &Ship)>()
    {
        sp.0 *= 0.99;
        pos.0 += dir.0.normalize() * sp.0 * delta_time;
        translate_coord(pos);
    }
    rotation_system(world, delta_time);
}

fn rotation_system(world: &mut World, delta_time: f32) {
    asteroid_rotation(world, delta_time);
    for (_, (dir, rot)) in world.query_mut::<(&mut Direction, &Rotation)>() {
        dir.0 = Vec2::from_angle(rot.0 - PI / 2.).normalize_or_zero();
    }
}

fn asteroid_rotation(world: &mut World, delta_time: f32) {
    for (_, (rot, _)) in world.query_mut::<(&mut Rotation, &Asteroid)>() {
        rot.0 += 0.3 * delta_time;
    }
}
