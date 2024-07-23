use std::ops::{Add, Mul, Sub};

use nannou::prelude::*;

use crate::{grid::{self, Grid}, CELLSIZE, DAMPING, GRAVITY, HEIGHT, MAX_ACC, WIDTH};

pub struct VerletObject {
    pub current: Vec2,
    pub previous: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
}

impl VerletObject {
    pub fn update(&mut self, dt: f32) {
        let disp = Vec2::sub(self.current, self.previous);
        self.previous = self.current;
        self.acceleration = Vec2::mul(self.acceleration, dt*dt);
        self.current = Vec2::add(self.current, disp);
        //
        self.acceleration = Vec2::new(clamp(self.acceleration.x,-MAX_ACC,MAX_ACC ), clamp(self.acceleration.y, -MAX_ACC, MAX_ACC));
        //
        self.current = Vec2::add(self.current, self.acceleration);
        self.acceleration = Vec2::ZERO;

    }
    pub fn apply_force(&mut self, force: Vec2) {
        let tmp = Vec2::add(self.acceleration, force);
        self.acceleration = Vec2::new(clamp(tmp.x,-MAX_ACC,MAX_ACC ), clamp(tmp.y, -MAX_ACC, MAX_ACC));
        // self.acceleration = Vec2::add(self.acceleration, force);
    }
    pub fn check_bounds(&mut self) {
        
        if self.current.x > WIDTH {
            let disp = self.current.x - self.previous.x;
            self.current.x = WIDTH;
            self.previous.x = self.current.x + disp/DAMPING;
        }
        if self.current.x < -WIDTH {
            let disp = self.current.x - self.previous.x;
            self.current.x = -WIDTH;
            self.previous.x = self.current.x + disp/DAMPING;
        }
        if self.current.y > HEIGHT {
            let disp = self.current.y - self.previous.y;
            self.current.y = HEIGHT;
            self.previous.y = self.current.y + disp/DAMPING;
        }
        if self.current.y < -HEIGHT {
            let disp = self.current.y - self.previous.y;
            self.current.y = -HEIGHT;
            self.previous.y = self.current.y + disp/DAMPING;
        }
    }

    fn handle_collision(objects: &mut Vec<VerletObject>, i: usize, j: usize) {
        let axis = Vec2::sub(objects[i].current, objects[j].current);
        let dist = Vec2::length(axis);
        let min_dist = objects[i].radius + objects[j].radius;
        if dist < min_dist {
            let norm = Vec2::normalize(axis);
            let delta = objects[i].radius + objects[j].radius - dist;
            let norm = Vec2::mul(norm, delta/2.0);
            objects[i].current = Vec2::add(objects[i].current, norm);
            objects[j].current = Vec2::sub(objects[j].current, norm);
        }
    }

    pub fn handle_env_collision(objects: &mut Vec<VerletObject>,env_objects: &mut Vec<VerletObject>,i: usize, j: usize) {
        let axis = Vec2::sub(objects[i].current, env_objects[j].current);
        let dist = Vec2::length(axis);
        let min_dist = objects[i].radius + env_objects[j].radius;
        if dist < min_dist {
            let norm = Vec2::normalize(axis);
            let delta = objects[i].radius + env_objects[j].radius - dist;
            let norm = Vec2::mul(norm, delta/2.0);
            objects[i].current = Vec2::add(objects[i].current, norm);
        }
    }

    pub fn check_collisions(objects: &mut Vec<VerletObject>, grid: &Grid) {
        for i in 0..objects.len() {
            // if grid.objects[objects[i].get_grid_index()].len() == 0 {
            //     break;
            // }
            for k in objects[i].adjacent_cells().iter() {
                for j in 0..grid.objects[*k].len() {
                    let jdx = grid.objects[*k][j];
                    if i == jdx {
                        continue;
                    }
                    VerletObject::handle_collision(objects,i,jdx);
                }
            }
        }
    }

    pub fn check_env_collision(objects: &mut Vec<VerletObject>, env_objects: &mut Vec<VerletObject>, grid: &Grid) {
        
        for env in 0..env_objects.len() {
            // if grid.objects[env_objects[env].get_grid_index()].len() == 0 {
            //     println!("{:?}", grid.objects[env_objects[env].get_grid_index()].len());
            //     break;
            // }
            for adjacent_index in env_objects[env].adjacent_cells().iter() {
                for j in 0..grid.objects[*adjacent_index].len() {
                    let jdx = grid.objects[*adjacent_index][j];
                    VerletObject::handle_env_collision(objects,env_objects,jdx,env);
                }
            }
        }
        
    }

    pub fn apply_force_point(objects: &mut Vec<VerletObject>, pos:Vec2, direction:Vec2) {
        let force = Vec2::new(50.0, 50.0 - GRAVITY);
        if pos.x < -WIDTH || pos.x > WIDTH || pos.y < -HEIGHT || pos.y > HEIGHT {
            return;
        }
        for i in 0..objects.len() {
            let axis = Vec2::sub(objects[i].current, pos);
            let dist = Vec2::length(axis);
            let norm = Vec2::normalize(axis);
            let norm = Vec2::mul(norm, dist);
            let norm = Vec2::mul(norm, direction);
            objects[i].apply_force(Vec2::mul(force, norm))
        }
    }
    pub fn get_grid_index(&self) -> usize {
        let x = ((self.current.x+WIDTH)/CELLSIZE) as usize;
        let y = ((self.current.y+HEIGHT)/CELLSIZE) as usize;
        x + y*CELLSIZE as usize
    }
    fn adjacent_cells(&self) -> Vec<usize> {
        let mut cells = Vec::new();
        
        for i in -1..2 {
            for j in -1..2 {
                let x = ((self.current.x+WIDTH+(CELLSIZE*i as f32))/CELLSIZE) as usize;
                let y = ((self.current.y+HEIGHT+(CELLSIZE*j as f32))/CELLSIZE) as usize;
                let idx = x + y*CELLSIZE as usize;
                if x >= 0 && x < (WIDTH*2.0) as usize && y >= 0 && y < (HEIGHT*2.0) as usize && !cells.contains(&idx) {
                    cells.push(idx);
                }
            }
        }
        
        cells
    }
}
    
