use macroquad::prelude::*;

pub struct Health(pub u32);
pub struct CurrentHealth(pub u32);

pub struct Damage(pub u32);

#[derive(Clone)]
pub struct Position(pub Vec2);

#[derive(Clone)]
pub struct Direction(pub Vec2);

pub struct Speed(pub f32);
pub struct Rotation(pub f32);

pub struct Gun;

pub struct Asteroid;
pub struct Ship;
pub struct Bullet;

pub struct LastShootTime(pub f64);

pub struct Size(pub f32);

pub struct ReloadTime(pub f64);

pub struct FlyText {
    pub text: String,
    pub create_time: f64,
    pub duration: f64,
}
