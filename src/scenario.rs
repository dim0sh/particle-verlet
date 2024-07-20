use crate::{app::Model, verlet::{self, VerletObject}, MAX_PARTICLES};
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

pub fn env_drizzle(model: &mut Model, update: Update) {
    drizzle(model, update);
    if model.initial {
        spawn_cup(Vec2::new(0.0, 0.0),&mut model.env_objects, 7, 5.0);
        spawn_line(Vec2::new(50.0,-20.0),&mut model.env_objects, 7, 5.0);
    }
}

pub fn spawn_cup(pos:Vec2,env_objects:&mut Vec<VerletObject>,n:usize,line_weight:f32) {
    for i in 0..n {
        let new_pos = Vec2::new(pos.x + line_weight * (i as f32 - (n/2) as f32) ,pos.y + line_weight * abs(i as f32 - (n/2) as f32));
        env_objects.push(VerletObject {
            current: new_pos,
            previous: new_pos,
            acceleration: Vec2::new(0.0,0.0),
            radius: line_weight,
        })
    }
}

pub fn spawn_line(pos:Vec2,env_objects:&mut Vec<VerletObject>,n:usize,line_weight:f32) {
    for i in 0..n {
        let new_pos = Vec2::new(pos.x + line_weight * (i as f32 - (n/2) as f32) ,pos.y + line_weight * (i as f32 - (n/2) as f32));
        env_objects.push(VerletObject {
            current: new_pos,
            previous: new_pos,
            acceleration: Vec2::new(0.0,0.0),
            radius: line_weight,
        })
    }
}