use crate::simulation::particles::particle_vec::ParticleVec;


pub struct DistanceConstraint {
    pub d: f32,
    pub i1: usize,
    pub i2: usize,
    pub stable: bool,
    pub enabled: bool,
}

impl DistanceConstraint {

    // stable default = false
    pub fn new(d: f32, i1: usize, i2: usize, stable: bool) -> Self {
        Self {
            d,
            i1,
            i2,
            stable,
            enabled: true,
        }
    }

    pub fn from_particles(i1: usize, i2: usize, particles: &ParticleVec) -> Self {
        let d = (particles[i1].pos - particles[i2].pos).magnitude();
        Self::new(d, i1, i2, false)
    }

    pub fn project(&self, estimates: &mut ParticleVec, counts: &Vec<usize>) {
        if !self.enabled {
            return;
        }

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

pub struct DistanceConstraintVec(pub Vec<DistanceConstraint>);

impl DistanceConstraintVec {
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

    pub fn push(&mut self, c: DistanceConstraint) {
        self.0.push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::math::vec2::Vec2, simulation::particles::particle::Particle};

    #[test]
    fn test_distance_disabled() {
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

        // Rest length 5.0, current distance 10.0. Should contract if enabled.
        let mut constraint = DistanceConstraint::new(5.0, 0, 1, false);
        constraint.enabled = false;
        
        let counts = vec![1, 1];

        constraint.project(&mut particles, &counts);

        let new_dist = (particles[0].pos_guess - particles[1].pos_guess).magnitude();
        assert_eq!(new_dist, 10.0); // Should not have moved
    }
}