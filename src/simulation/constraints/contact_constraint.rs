use crate::{core::math::vec2::Vec2, simulation::particles::{body::Body, particle_vec::ParticleVec}};


pub struct ContactConstraint {
    pub i1: usize,
    pub i2: usize,
    pub stable: bool,
}

impl ContactConstraint {
    pub fn new(i1: usize, i2: usize, stable: bool) -> Self {
        Self {
            i1,
            i2,
            stable
        }
    }

    pub fn project(&self, estimates: &mut ParticleVec, counts: &Vec<usize>) {
        let p1 = estimates[self.i1];
        let p2 = estimates[self.i2];
        
        if p1.tmass == 0.0 && p2.tmass == 0.0 {
            return;
        }

        let diff = p1.get_p(self.stable) - p2.get_p(self.stable);
        let w_sum = p1.tmass + p2.tmass;
        let dist = diff.magnitude();
        let particle_diam = p1.radius + p2.radius;
        let mag = dist - particle_diam;

        // Previous iterations have moved particles out of collision
        if mag > 0.0 {
            return;
        }

        let scale = mag / w_sum;

        let dp = (scale / dist) * diff;
        let dp1 = -p1.tmass * dp / counts[self.i1] as f32;
        let dp2 = p2.tmass * dp / counts[self.i2] as f32;

        estimates[self.i1].pos_guess += dp1;
        estimates[self.i2].pos_guess += dp2;

        if self.stable {
            estimates[self.i1].pos += dp1;
            estimates[self.i2].pos += dp2;
        }
    }

    pub fn update_counts(&self, counts: &mut Vec<usize>) {
        counts[self.i1] += 1;
        counts[self.i2] += 1;
    }
}

pub struct ContactConstraintVec(pub Vec<ContactConstraint>);

impl ContactConstraintVec {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn update_counts(&self, counts: &mut Vec<usize>) {
        for c in &self.0 {
            c.update_counts(counts);
        }
    }

    pub fn solve(&self, particles: &mut ParticleVec, counts: &Vec<usize>) {
        for c in &self.0 {
            c.project(particles, counts);
        }
    }

    pub fn push(&mut self, c: ContactConstraint) {
        self.0.push(c);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, ContactConstraint> {
        self.0.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}