use std::{env, ops::{Add, Mul, Sub}};

use nannou::prelude::*;

use crate::{DAMPING, HEIGHT, MAX_ACC, WIDTH};

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
        self.current = Vec2::add(self.current, self.acceleration);
        self.acceleration = Vec2::ZERO;

    }
    pub fn apply_force(&mut self, force: Vec2) {
        let tmp = Vec2::add(self.acceleration, force);
        self.acceleration = Vec2::new(clamp(tmp.x,-MAX_ACC,MAX_ACC ), clamp(tmp.y, -MAX_ACC, MAX_ACC));
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

    pub fn check_collisions(objects: &mut Vec<VerletObject>) {
        for i in 0..objects.len() {
            for j in i+1..objects.len() {
                VerletObject::handle_collision(objects, i, j);
            }
        }
    }

    pub fn check_env_collision(objects: &mut Vec<VerletObject>, env_objects: &mut Vec<VerletObject>) {
        for i in 0..objects.len() {
            for j in 0.. env_objects.len() {
                VerletObject::handle_env_collision(objects,env_objects,i,j);
            }
        }
    }
}
    
