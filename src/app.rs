use crate::{scenario, verlet::{self, VerletObject}, GRAVITY, MAX_PARTICLES, SUB_TICK};
use nannou::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum State {
    Running,
    Paused,
    Restart,
}

impl State {
    fn invert_pause(&mut self) {
        match self {
            State::Running => *self = State::Paused,
            State::Paused => *self = State::Running,
            State::Restart => {},
        }
    }
}

pub struct Model {
    pub initial: bool,
    pub objects: Vec<verlet::VerletObject>,
    pub env_objects: Vec<verlet::VerletObject>,
    state: State,
    pub timer: f32,
    pub spawn_switch: f32,
}

// fn init(n:usize) -> Vec<verlet::VerletObject> {
//     (0..n).into_iter().map(|i| {
//         verlet::VerletObject {
//             current: Vec2::new(200.0 - i as f32 * 3.,200.0),
//             previous: Vec2::new(200.0, 200.0),
//             acceleration: Vec2::new(random_range(-2., 2.), random_range(-2., 2.)),
//             radius: 2.0,
//         }
//     }).collect()
// }

pub fn model(app: &App) -> Model {
    app.new_window()
    .size(800,800)
    .event(window_event)
    .view(view)
    .build()
    .unwrap();
    
    Model {
        initial: true,
        objects: Vec::new(),
        env_objects: Vec::new(),
        state: State::Paused,
        timer: 0.0,
        spawn_switch: 0.0,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.objects.iter().for_each(|object| {
        draw.ellipse()
            .resolution(8.0)
            .x_y(object.current.x, object.current.y)
            .radius(object.radius)
            .color(WHITE);
    });
    model.env_objects.iter().for_each(|object| {
        draw.ellipse()
            .resolution(8.0)
            .x_y(object.current.x, object.current.y)
            .radius(object.radius)
            .color(RED);
    });
    draw.to_frame(app, &frame).unwrap();
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            match key {
                Key::Space => model.state.invert_pause(),
                Key::R => model.state = State::Restart,
                _ => (),
            }
        }, 
        _ => (),
    }
}

pub fn update(_app: &App, model: &mut Model, update: Update) {
    if model.state == State::Paused {
        return;
    }
    if model.state == State::Restart {
        model.objects = Vec::new();
        model.state = State::Paused;
    }
    // scenario::drizzle(model, update);
    scenario::env_drizzle(model, update);
    model.initial = false;
    for _ in 0..SUB_TICK {
        model.objects.iter_mut().for_each(|object| {
            object.apply_force(Vec2::new(0.0, GRAVITY));
            object.check_bounds();
            object.update(0.1/SUB_TICK as f32);
        });
        
        verlet::VerletObject::check_collisions(&mut model.objects);
        verlet::VerletObject::check_env_collision(&mut model.objects, &mut model.env_objects);
    }
}