use crate::core::components::*;
use hecs::{Entity, With, World};
use macroquad::time::get_time;

pub fn bullets_and_asteroids_collision(world: &mut World) {
    let mut collided: Vec<Entity> = Vec::new();

    for (id0, (pos, dmg, b_size)) in
        &mut world.query::<With<(&Position, &Damage, &Size), &Bullet>>()
    {
        for (a_id, (a_pos, a_size)) in &mut world.query::<With<(&Position, &Size), &Asteroid>>() {
            if (a_pos.0 - pos.0).length() <= b_size.0 + a_size.0 {
                collided.push(id0);
                let mut asteroid_hlth = world.get::<&mut CurrentHealth>(a_id).unwrap();
                let mut asteroid_ft = world.get::<&mut Option<FlyText>>(a_id).unwrap();
                asteroid_ft.replace(FlyText {
                    text: String::from(format!("-{}", dmg.0)),
                    create_time: get_time(),
                    duration: 0.5,
                });
                asteroid_hlth.0 = asteroid_hlth.0.checked_sub(dmg.0).unwrap_or(0);
                if asteroid_hlth.0 <= 0 {
                    collided.push(a_id);
                }
            }
        }
    }

    delete_entites(world, collided);
}

pub fn ship_and_asteroids_collision(world: &mut World, ship: Entity) {
    let mut collided = Vec::new();
    {
        let ship_pos = world.get::<&Position>(ship).unwrap();
        let mut ship_health = world.get::<&mut CurrentHealth>(ship).unwrap();
        let mut ship_ft = world.get::<&mut Option<FlyText>>(ship).unwrap();

        for (ast, (pos, dmg, size)) in
            &mut world.query::<With<(&Position, &Damage, &Size), &Asteroid>>()
        {
            if (ship_pos.0 - pos.0).length() <= 3. + size.0 {
                ship_health.0 = ship_health.0.checked_sub(dmg.0).unwrap_or(0);
                ship_ft.replace(FlyText {
                    text: String::from(format!("-{}", dmg.0)),
                    create_time: get_time(),
                    duration: 0.5,
                });
                collided.push(ast);
            }
        }
    }
    delete_entites(world, collided);
}

pub fn delete_entites(world: &mut World, collided: Vec<Entity>) {
    for ent in collided {
        world.despawn(ent);
    }
}
