use crate::{math::vec2::Vec2, particles::{body::Body, particle_vec::ParticleVec}};


pub struct TotalShapeConstraint {
}

impl TotalShapeConstraint {
    pub fn new() -> Self {
        Self {
        }
    }

    fn guess(&self, idx: usize, body: &Body) -> Vec2 {
        let c = body.angle.cos();
        let s = body.angle.sin();

        let q = match body.rs.get(&idx) {
            Some(value) => *value,
            None => Vec2::new(0.0, 0.0)
        };

        let d = Vec2::new(c * q.x - s * q.y, s * q.x + c * q.y);
        return d + body.center;
    }

    pub fn project(&self, estimates: &mut ParticleVec, counts: &Vec<usize>, body: &mut Body) {
        body.update_com(estimates, true);

        // implemented using http://labs.byhook.com/2010/06/29/particle-based-rigid-bodies-using-shape-matching/
        for i in 0..body.particle_indicies.len() {
            let idx = body.particle_indicies[i];
            let p = &mut estimates[idx];
            p.pos_guess += (self.guess(idx, body) - p.pos_guess) * body.stiffness;
        }
    }

    pub fn update_counts(&self, counts: &mut Vec<usize>, body: &Body) {
        for i in 0..body.particle_indicies.len() {
            counts[body.particle_indicies[i]] += 1;
        }
    }
}