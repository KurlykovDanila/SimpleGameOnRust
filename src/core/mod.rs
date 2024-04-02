use hecs::{With, World};

use crate::{Asteroid, FlyText};

pub mod collision_system;
pub mod components;
pub mod motion_system;
pub mod render_system;
pub mod spawn_systems;

pub fn has_asteroids(world: &mut World) -> bool {
    for _ in world.query_mut::<&Asteroid>() {
        return true;
    }
    return false;
}

pub fn clear_fly_text(world: &mut World, time_now: f64) {
    for (_, ft) in world.query_mut::<&mut Option<FlyText>>() {
        if let Some(t) = ft {
            if time_now - t.create_time >= t.duration {
                ft.take();
            }
        }
    }
}
