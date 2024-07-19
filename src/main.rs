use nannou;
mod app;
mod verlet;
mod scenario;

use app::model;
use app::update;

pub const MAX_PARTICLES: usize = 1000;

pub const WIDTH: f32 = 200.0;
pub const HEIGHT: f32 = 200.0;

pub const GRAVITY: f32 = -98.1;
pub const MAX_ACC: f32 = 100.0;

pub const DAMPING: f32 = 1.1;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
