use crate::{grid, scenario, verlet, CELLSIZE, GRAVITY, SUB_TICK,WIDTH,HEIGHT};
use nannou::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Running,
    Paused,
    Restart,
    Init,
}

impl State {
    fn invert_pause(&mut self) {
        match self {
            State::Running => *self = State::Paused,
            State::Paused => *self = State::Running,
            State::Restart => {},
            State::Init => {},
        }
    }
}

pub struct Model {
    repel: bool,
    attract: bool,
    direction: Vec2,
    mouse_pos: Point2,
    pub objects: Vec<verlet::VerletObject>,
    pub env_objects: Vec<verlet::VerletObject>,
    pub state: State,
    pub timer: f32,
    pub spawn_switch: f32,
}

pub fn model(app: &App) -> Model {
    app.new_window()
    .size(800,800)
    .event(window_event)
    .mouse_moved(mouse_moved)
    .view(view)
    .build()
    .unwrap();
    
    Model {
        repel: false,
        attract: false,
        direction: Vec2::new(1.0,1.0),
        mouse_pos: Point2::new(0.0, 0.0),
        objects: Vec::new(),
        env_objects: Vec::new(),
        state: State::Init,
        timer: 0.0,
        spawn_switch: 0.0,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.line().start(Point2::new(-WIDTH, HEIGHT)).end(Point2::new(WIDTH, HEIGHT)).color(RED);
    draw.line().start(Point2::new(-WIDTH, -HEIGHT)).end(Point2::new(WIDTH, -HEIGHT)).color(RED);
    draw.line().start(Point2::new(-WIDTH, HEIGHT)).end(Point2::new(-WIDTH, -HEIGHT)).color(RED);
    draw.line().start(Point2::new(WIDTH, HEIGHT)).end(Point2::new(WIDTH, -HEIGHT)).color(RED);


    draw.rect().w_h(CELLSIZE, CELLSIZE).color(BLACK);
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
                Key::A => {model.attract = !model.attract;model.repel = false},
                Key::D => {model.repel = !model.repel;model.attract = false;},
                _ => (),
            }
        }, 
        _ => (),
    }
}

pub fn mouse_moved(_app: &App, model: &mut Model, pos:Point2) {
    model.mouse_pos = pos;
}

pub fn update(_app: &App, model: &mut Model, update: Update) {
    
    if model.state == State::Paused {
        return;
    }
    if model.state == State::Restart {
        model.objects = Vec::new();
        model.state = State::Paused;
    }
    
    scenario::env_drizzle(model, update);
    
    if model.state == State::Init {
        model.state = State::Paused;
    }    

    
    for _ in 0..SUB_TICK {
        
        model.objects.iter_mut().for_each(|object| {
            object.apply_force(Vec2::new(0.0, GRAVITY));
            object.check_bounds();
            object.update(0.1/SUB_TICK as f32);
        });
        
        if model.attract {
            verlet::VerletObject::apply_force_point(&mut model.objects, model.mouse_pos, -model.direction);
        }
        if model.repel {
            verlet::VerletObject::apply_force_point(&mut model.objects, model.mouse_pos, model.direction);
        }
        let grid = grid::Grid::new(&model.objects);
        verlet::VerletObject::check_collisions(&mut model.objects, &grid);
        verlet::VerletObject::check_env_collision(&mut model.objects, &mut model.env_objects);
    }
}