use std::borrow::Borrow;

use hecs::{Entity, With, World};
use macroquad::prelude::*;

use crate::core::components::*;

pub fn ship_render(world: &mut World, ship: Entity, time_now: f64) {
    const SHIP_HEIGHT: f32 = 25.;
    const SHIP_BASE: f32 = 22.;
    let pos = world.get::<&Position>(ship).unwrap();
    let rotation = world.get::<&Rotation>(ship).unwrap();
    let max_hp = world.get::<&Health>(ship).unwrap();
    let c_hp = world.get::<&CurrentHealth>(ship).unwrap();
    let lst = world.get::<&LastShootTime>(ship).unwrap();
    let rt = world.get::<&ReloadTime>(ship).unwrap();
    let ft = world.get::<&Option<FlyText>>(ship).unwrap();
    let v1 = Vec2::new(
        pos.0.x + rotation.0.sin() * SHIP_HEIGHT / 2.,
        pos.0.y - rotation.0.cos() * SHIP_HEIGHT / 2.,
    );
    let v2 = Vec2::new(
        pos.0.x - rotation.0.cos() * SHIP_BASE / 2. - rotation.0.sin() * SHIP_HEIGHT / 2.,
        pos.0.y - rotation.0.sin() * SHIP_BASE / 2. + rotation.0.cos() * SHIP_HEIGHT / 2.,
    );
    let v3 = Vec2::new(
        pos.0.x + rotation.0.cos() * SHIP_BASE / 2. - rotation.0.sin() * SHIP_HEIGHT / 2.,
        pos.0.y + rotation.0.sin() * SHIP_BASE / 2. + rotation.0.cos() * SHIP_HEIGHT / 2.,
    );
    draw_triangle_lines(v1, v2, v3, 2., WHITE);
    ship_health_render(pos.borrow(), max_hp.borrow(), c_hp.borrow());
    ship_reloading_render(pos.borrow(), lst.borrow(), rt.borrow(), time_now);
    if let Some(t) = ft.as_ref() {
        fly_text_render(&t, pos.borrow(), &Size(30.), ORANGE, get_time());
    }
}

pub fn asteroid_render(world: &mut World) {
    for (_, (pos, size, max_hp, c_hp, ft)) in &mut world
        .query::<With<(&Position, &Size, &Health, &CurrentHealth, &Option<FlyText>), &Asteroid>>()
    {
        draw_circle(pos.0.x, pos.0.y, size.0, GRAY);
        asteroid_health_render(pos, max_hp, c_hp, size);
        if let Some(t) = ft {
            fly_text_render(t, pos, size, RED, get_time());
        }
    }
}

pub fn bullet_render(world: &mut World) {
    for (_, (pos, size)) in &mut world.query::<With<(&Position, &Size), &Bullet>>() {
        draw_circle(pos.0.x, pos.0.y, size.0, WHITE);
    }
}

fn asteroid_health_render(pos: &Position, max_hp: &Health, c_hp: &CurrentHealth, size: &Size) {
    let health_bar_length = 20.;
    let x = pos.0.x - health_bar_length / 2.;
    let y = pos.0.y - size.0 - 5.;
    let hp_percent = c_hp.0 as f32 / max_hp.0 as f32;
    draw_line(x - 2., y, x + health_bar_length + 2., y, 7., WHITE);
    draw_line(x, y, x + health_bar_length * hp_percent, y, 5., RED);
}

fn ship_health_render(pos: &Position, max_hp: &Health, c_hp: &CurrentHealth) {
    let health_bar_length = 40.;
    let x = pos.0.x - health_bar_length / 2.;
    let y = pos.0.y - 20.;
    let hp_percent = c_hp.0 as f32 / max_hp.0 as f32;
    draw_line(x - 2., y, x + health_bar_length + 2., y, 9., WHITE);
    draw_line(x, y, x + health_bar_length * hp_percent, y, 5., GREEN);
}

fn ship_reloading_render(pos: &Position, lst: &LastShootTime, rt: &ReloadTime, time_now: f64) {
    let reloading_bar_length = 40.;
    let x = pos.0.x - reloading_bar_length / 2.;
    let y = pos.0.y - 30.;
    let reloading_percent = (time_now - lst.0).min(rt.0) / rt.0;
    draw_line(x - 2., y, x + reloading_bar_length + 2., y, 9., WHITE);
    draw_line(
        x,
        y,
        x + reloading_bar_length * reloading_percent as f32,
        y,
        5.,
        YELLOW,
    );
}

fn fly_text_render(ft: &FlyText, pos: &Position, size: &Size, color: Color, time_now: f64) {
    let x = pos.0.x - size.0 * 1.3 - 20.;
    let y = pos.0.y - size.0;

    let coef = 1. - (time_now - ft.create_time) / ft.duration;
    let font_size = 40. * coef.max(0.2).cos();
    let color = Color::new(color.r, color.g, color.b, color.a * coef.max(0.1) as f32);
    draw_text(&ft.text, x, y, font_size as f32, color);
}
