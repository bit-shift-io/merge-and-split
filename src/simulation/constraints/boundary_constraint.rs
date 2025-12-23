use crate::{core::math::vec2::Vec2, simulation::particles::{particle::Phase, particle_vec::ParticleVec}};

pub struct BoundaryConstraint {
    pub index: usize,
    pub value: f32,
    pub x_boundary: bool,
    pub greater: bool,
    pub stable: bool,
}

impl BoundaryConstraint {
    pub fn new(index: usize, value: f32, x_boundary: bool, greater: bool, stable: bool) -> Self {
        Self {
            index,
            value,
            x_boundary,
            greater, 
            stable,
        }
    }

    pub fn project(&self, estimates: &mut ParticleVec, counts: &Vec<usize>) {
        let p = &mut estimates[self.index];

        // Add a little random jitter for fluids and gases so particles do not become trapped on boundaries
        let extra = if p.phase == Phase::Fluid || p.phase == Phase::Gas { 0.0 /* todo: procedural random frand() * .003 */ } else { 0.0 };
        let d = p.radius + extra;

        // Move the particle back into a valid spot (if necessary)
        let n = if self.greater {
            if self.x_boundary {

                // Quit if no longer valid
                if p.pos_guess.x >= self.value + p.radius {
                    return;
                }
                p.pos_guess.x = self.value + d;
                if self.stable {
                    p.pos.x = self.value + d;
                }
                Vec2::new(1.0, 0.0)
            } else {

                // Quit if no longer valid
                if p.pos_guess.y >= self.value + p.radius {
                    return;
                }
                p.pos_guess.y = self.value + d;
                if self.stable {
                    p.pos.y = self.value + d;
                }
                Vec2::new(0.0, 1.0)
            }
        } else {
            if self.x_boundary {

                // Quit if no longer valid
                if p.pos_guess.x <= self.value - p.radius {
                    return;
                }
                p.pos_guess.x = self.value - d;
                if self.stable {
                    p.pos.x = self.value - d;
                }
                Vec2::new(-1.0, 0.0)
            } else {

                // Quit if no longer valid
                if p.pos.y <= self.value - p.radius {
                    return;
                }
                p.pos_guess.y = self.value - d;
                if self.stable {
                    p.pos.y = self.value - d;
                }
                Vec2::new(0.0,-1.0)
            }
        };

        if self.stable {
            return;
        }

        // Apply friction - boundaries have a coefficient of friction of 1
        let dp = (p.pos_guess - p.pos) / (counts[self.index] as f32);
        let dpt = dp - dp.dot(n) * n;
        let ldpt = dpt.magnitude();

        if ldpt < f32::EPSILON {
            return;
        }

        // Choose between static and kinetic friction
        if ldpt < p.s_friction.sqrt() * d {
            p.pos_guess -= dpt;
        } else {
            p.pos_guess -= dpt * f32::min(p.k_friction.sqrt() * d / ldpt, 1.);
        }
    }

    pub fn evaluate(&self, _estimates: &ParticleVec) {

    }


    pub fn gradient(&self, _estimates: &ParticleVec, _respect: i32) -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    pub fn update_counts(&self, counts: &mut Vec<usize>) {
        counts[self.index] += 1; 
    }
}

pub struct BoundaryConstraintVec(pub Vec<BoundaryConstraint>);

impl BoundaryConstraintVec {
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

    pub fn push(&mut self, c: BoundaryConstraint) {
        self.0.push(c);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, BoundaryConstraint> {
        self.0.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}