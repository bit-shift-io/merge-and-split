use std::collections::HashMap;

use cgmath::InnerSpace;
use std::f32::consts::PI;

use crate::{core::math::vec2::Vec2, simulation::particles::{particle_vec::ParticleVec, sdf_data::SdfData}};

pub struct Body {
    pub particle_indicies: Vec<usize>,
    pub center: Vec2,
    pub imass: f32,
    pub angle: f32,
    pub rs: HashMap<usize, Vec2>, // map from global particles index to r vector
    pub sdf: HashMap<usize, SdfData>, // map from global particles index to SDF data

    pub stiffness: f32, // for the TotalShapeConstraint
}

impl Body {
    pub fn new() -> Self {
        Self {
            particle_indicies: vec![],
            center: Vec2::new(0.0, 0.0),
            imass: 0.0,
            angle: 0.0,
            rs: HashMap::new(),
            sdf: HashMap::new(),

            stiffness: 1.0,
        }
    }

    pub fn update_com(&mut self, estimates: &ParticleVec, use_estimates: bool) {
        // Recompute center of mass
        let mut total = Vec2::new(0.0, 0.0);
        for i in 0..self.particle_indicies.len() {
            let p = estimates[self.particle_indicies[i]];
            if use_estimates {
                total += p.pos_guess / p.imass;
            } else {
                total += p.pos / p.imass;
            }
        }
        self.center = total * self.imass;

        // Recompute angle delta guess
        self.angle = 0.0;
        let mut prev = 0.0;
        for i in 0..self.particle_indicies.len() {
            let index = self.particle_indicies[i];

            let q = match self.rs.get(&index) {
                Some(value) => *value,
                None => Vec2::new(0.0, 0.0)
            };

            if q.dot(q) == 0.0 {
                continue;
            }
            let p = estimates[index];
            let r = p.pos_guess - self.center;

            let cos = r.x * q.x + r.y * q.y;
            let sin = r.y * q.x - r.x * q.y;
            let mut next = sin.atan2(cos);

            // Ensure all guesses are close to each other
            if i > 0 {
                if prev - next >= PI {
                    next += 2.0 * PI;
                }
            } else {
                if next < 0.0 {
                    next += 2.0 * PI;
                }
            }

            prev = next;
            next /= p.imass;
            self.angle += next;
        }
        self.angle *= self.imass;
    }

    pub fn compute_rs(&mut self, estimates: &ParticleVec) {
        let mut imass = 0.0;
        for i in 0..self.particle_indicies.len() {
            let idx = self.particle_indicies[i];
            let p = estimates[idx];
            let r = p.pos - self.center;
            self.rs.insert(idx, r);

            if r.dot(r) != 0.0 {
                imass += 1.0 / p.imass;
            }
        }
        self.imass = 1.0 / imass;
    }

    pub fn for_each_particle<F>(&self, mut f: F)
    where
        F: FnMut(usize),
    {
        for i in 0..self.particle_indicies.len() {
            f(self.particle_indicies[i]);
        }
    }

    // Helper to add count for each particle in the body
    pub fn update_counts(&self, counts: &mut Vec<usize>, count: usize) {
        for i in 0..self.particle_indicies.len() {
            counts[self.particle_indicies[i]] += count;
        }
    }
}