use crate::{core::math::vec2::Vec2, simulation::particles::particle_vec::ParticleVec};

pub struct VolumeConstraint {
    pub rest_volume: f32,
    pub compliance: f32,
    pub particle_indices: Vec<usize>,
}

impl VolumeConstraint {
    pub fn new(compliance: f32, particle_indices: Vec<usize>, particles: &ParticleVec) -> Self {
        let mut constraint = Self {
            rest_volume: 0.0,
            compliance,
            particle_indices,
        };
        constraint.rest_volume = constraint.calculate_volume(particles, true); // Calculate initial volume as rest volume
        constraint
    }

    fn calculate_volume(&self, particles: &ParticleVec, use_pos: bool) -> f32 {
        let mut volume = 0.0;
        let n = self.particle_indices.len();
        for i in 0..n {
            let p1 = if use_pos { particles[self.particle_indices[i]].pos } else { particles[self.particle_indices[i]].pos_guess };
            let p2 = if use_pos { particles[self.particle_indices[(i + 1) % n]].pos } else { particles[self.particle_indices[(i + 1) % n]].pos_guess };
            volume += p1.x * p2.y - p2.x * p1.y;
        }
        volume * 0.5
    }

    pub fn project(&self, estimates: &mut ParticleVec, counts: &Vec<usize>, dt: f32) {
        let current_volume = self.calculate_volume(estimates, false);
        let c = current_volume - self.rest_volume;

        if c.abs() < f32::EPSILON {
            return;
        }

        let alpha_tilde = self.compliance / (dt * dt);
        
        let mut w_sum = 0.0;
        let mut grads = Vec::with_capacity(self.particle_indices.len());

        let n = self.particle_indices.len();
        for i in 0..n {
            let prev_idx = self.particle_indices[(i + n - 1) % n];
            let next_idx = self.particle_indices[(i + 1) % n];
            
            let p_prev = estimates[prev_idx].pos_guess;
            let p_next = estimates[next_idx].pos_guess;

            // Gradient of area with respect to p_i: 0.5 * (p_next - p_prev)^perp
            // perp(x, y) = (-y, x)
            let grad = Vec2::new(p_next.y - p_prev.y, p_prev.x - p_next.x) * 0.5;
            grads.push(grad);

            w_sum += estimates[self.particle_indices[i]].imass * grad.magnitude().powi(2);
        }

        if w_sum < f32::EPSILON {
            return;
        }

        let lambda = -c / (w_sum + alpha_tilde);

        for (i, &idx) in self.particle_indices.iter().enumerate() {
            let p = &mut estimates[idx];
            if p.imass > 0.0 {
                let delta = lambda * p.imass * grads[i];
                p.pos_guess += delta / counts[idx] as f32;
            }
        }
    }

    pub fn update_counts(&self, counts: &mut Vec<usize>) {
        for &idx in &self.particle_indices {
            counts[idx] += 1;
        }
    }
}

pub struct VolumeConstraintVec(pub Vec<VolumeConstraint>);

impl VolumeConstraintVec {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn update_counts(&self, counts: &mut Vec<usize>) {
        for c in &self.0 {
            c.update_counts(counts);
        }
    }

    pub fn solve(&mut self, particles: &mut ParticleVec, counts: &Vec<usize>, dt: f32) {
        for c in &mut self.0 {
            c.project(particles, counts, dt);
        }
    }

    pub fn push(&mut self, c: VolumeConstraint) {
        self.0.push(c);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, VolumeConstraint> {
        self.0.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::particles::particle::Particle;

    #[test]
    fn test_volume_calculation() {
        let mut particles = ParticleVec::new();
        // Create a 2x2 square
        let positions = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(0.0, 2.0),
        ];

        let mut indices = vec![];
        for (i, pos) in positions.iter().enumerate() {
            let mut p = Particle::default();
            p.pos = *pos;
            p.pos_guess = *pos;
            p.imass = 1.0;
            particles.push(p);
            indices.push(i);
        }

        let constraint = VolumeConstraint::new(0.0, indices, &particles);
        assert_eq!(constraint.rest_volume, 4.0);
    }

    #[test]
    fn test_volume_restoration() {
        let mut particles = ParticleVec::new();
        // Create a 2x2 square
        let positions = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(0.0, 2.0),
        ];

        let mut indices = vec![];
        for (i, pos) in positions.iter().enumerate() {
            let mut p = Particle::default();
            p.pos = *pos;
            p.pos_guess = *pos;
            p.imass = 1.0;
            particles.push(p);
            indices.push(i);
        }

        let constraint = VolumeConstraint::new(0.0, indices.clone(), &particles);
        
        // Deform the square (compress it)
        particles[2].pos_guess = Vec2::new(2.0, 1.0);
        particles[3].pos_guess = Vec2::new(0.0, 1.0);
        
        let mut counts = vec![1; 4];
        let dt = 0.016;

        // Apply constraint
        constraint.project(&mut particles, &counts, dt);

        // Check if volume increased back towards 4.0
        let new_volume = constraint.calculate_volume(&particles, false);
        assert!(new_volume > 2.0); // 2.0 is the compressed volume
    }
}
