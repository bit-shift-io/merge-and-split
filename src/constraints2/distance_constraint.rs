use cgmath::InnerSpace;

use crate::{math::vec2::Vec2, particles::{body::Body, particle_vec::ParticleVec}};


pub struct DistanceConstraint {
    pub d: f32,
    pub i1: usize,
    pub i2: usize,
    pub stable: bool,
}

impl DistanceConstraint {

    // stable default = false
    pub fn new(d: f32, i1: usize, i2: usize, stable: bool) -> Self {
        Self {
            d,
            i1,
            i2,
            stable
        }
    }

    pub fn from_particles(i1: usize, i2: usize, particles: &ParticleVec) -> Self {
        let d = (particles[i1].pos - particles[i2].pos).magnitude();
        Self::new(d, i1, i2, false)
    }

    pub fn project(&self, estimates: &mut ParticleVec, counts: &Vec<usize>) {
        let p1 = estimates[self.i1];
        let p2 = estimates[self.i2];
        
        if p1.imass == 0.0 && p2.imass == 0.0 {
            return;
        }

        let diff = p1.pos_guess - p2.pos_guess; //glm::dvec2 diff = p1->ep - p2->ep;
        let w_sum = p1.imass + p2.imass;
        let dist = diff.magnitude();
        let mag = dist - self.d;
        let scale = mag / w_sum;

        let dp = (scale / dist) * diff;
        let dp1 = -p1.imass * dp / counts[self.i1] as f32;
        let dp2 = p2.imass * dp / counts[self.i2] as f32;

        estimates[self.i1].pos_guess += dp1;
        estimates[self.i2].pos_guess += dp2;
    }

    pub fn update_counts(&self, counts: &mut Vec<usize>) {
        counts[self.i1] += 1;
        counts[self.i2] += 1;
    }
}