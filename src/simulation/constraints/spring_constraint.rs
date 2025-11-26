use crate::simulation::particles::particle_vec::ParticleVec;

pub struct SpringConstraint {
    pub d: f32,
    pub stiffness: f32,
    pub i1: usize,
    pub i2: usize,
    pub stable: bool,
}

impl SpringConstraint {
    // stable default = false
    pub fn new(d: f32, stiffness: f32, i1: usize, i2: usize, stable: bool) -> Self {
        Self {
            d,
            stiffness,
            i1,
            i2,
            stable,
        }
    }

    pub fn from_particles(i1: usize, i2: usize, particles: &ParticleVec, stiffness: f32) -> Self {
        let d = (particles[i1].pos - particles[i2].pos).magnitude();
        Self::new(d, stiffness, i1, i2, false)
    }

    pub fn project(&self, estimates: &mut ParticleVec, counts: &Vec<usize>, dt: f32) {
        let p1 = estimates[self.i1];
        let p2 = estimates[self.i2];

        if p1.imass == 0.0 && p2.imass == 0.0 {
            return;
        }

        let diff = p1.pos_guess - p2.pos_guess;
        let w_sum = p1.imass + p2.imass;
        let dist = diff.magnitude();
        
        // XPBD compliance (alpha_tilde)
        // alpha = 1 / (k * dt^2)
        // If stiffness is infinite, alpha is 0.
        // We use a small epsilon for dist to avoid division by zero if needed, but usually dist > 0 check or similar is good.
        // Here we just follow the standard formula.
        
        let alpha_tilde = 1.0 / (self.stiffness * dt * dt);
        
        let mag = dist - self.d;
        
        // XPBD scaling factor
        // lambda = -C / (w_sum + alpha_tilde)
        // correction = lambda * w * grad C
        // Here we calculate the scalar part of the correction
        
        let scale = mag / (w_sum + alpha_tilde);

        if dist < f32::EPSILON {
             return; // Avoid division by zero if particles are at the exact same position
        }

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

pub struct SpringConstraintVec(pub Vec<SpringConstraint>);

impl SpringConstraintVec {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn update_counts(&self, counts: &mut Vec<usize>) {
        for c in &self.0 {
            c.update_counts(counts);
        }
    }

    pub fn solve(&self, particles: &mut ParticleVec, counts: &Vec<usize>, dt: f32) {
        for c in &self.0 {
            c.project(particles, counts, dt);
        }
    }

    pub fn push(&mut self, c: SpringConstraint) {
        self.0.push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::math::vec2::Vec2, simulation::particles::particle::Particle};

    #[test]
    fn test_spring_contraction() {
        let mut particles = ParticleVec::new();
        
        let mut p1 = Particle::default();
        p1.pos = Vec2::new(0.0, 0.0);
        p1.pos_guess = Vec2::new(0.0, 0.0);
        p1.imass = 1.0;
        particles.push(p1);

        let mut p2 = Particle::default();
        p2.pos = Vec2::new(10.0, 0.0);
        p2.pos_guess = Vec2::new(10.0, 0.0);
        p2.imass = 1.0;
        particles.push(p2);

        // Rest length 5.0, current distance 10.0. Should contract.
        let spring = SpringConstraint::new(5.0, 100.0, 0, 1, false);
        
        let mut counts = vec![1, 1];
        let dt = 0.016; // 60fps

        spring.project(&mut particles, &counts, dt);

        let new_dist = (particles[0].pos_guess - particles[1].pos_guess).magnitude();
        assert!(new_dist < 10.0);
        assert!(new_dist > 5.0); // It shouldn't snap instantly with finite stiffness
    }
}
