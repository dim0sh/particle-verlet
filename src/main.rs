use nannou;
mod app;
mod verlet;
mod scenario;

use app::model;
use app::update;

pub const MAX_PARTICLES: usize = 100;

pub const WIDTH: f32 = 200.0;
pub const HEIGHT: f32 = 200.0;

pub const GRAVITY: f32 = -98.1;
pub const MAX_ACC: f32 = 100.0;

pub const DAMPING: f32 = 1.1;

pub const SUB_TICK: usize = 10;

pub const CIRCLE_RES: f32 = 2.0;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
