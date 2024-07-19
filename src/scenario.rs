use crate::{verlet,MAX_PARTICLES,app::Model};
use nannou::prelude::*;

pub fn drizzle(model: &mut Model, update: Update) {
    model.timer += update.since_last.as_secs_f32();
    if model.objects.len() < MAX_PARTICLES && model.timer > 0.2 {
        model.timer = 0.0;
        model.spawn_switch = (model.spawn_switch + 1.0)%3.0;
        let x = (model.spawn_switch-1.0) * 5.0;
        model.objects.push(verlet::VerletObject {
            current: Vec2::new(x, 200.0),
            previous: Vec2::new(x, 200.0),
            acceleration: Vec2::new(random_range(-20., 20.), random_range(-2., 2.)),
            radius: 2.0,
        });
    }
}