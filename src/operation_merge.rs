use cgmath::InnerSpace;

use crate::{operation::Operation, particle::Particle, particle_system::ParticleSystem};



pub struct OperationMerge {
}

impl Operation for OperationMerge {
    fn execute(&self, ps: &mut ParticleSystem) {
        let particle_count: usize = ps.len();
        for ai in 0..particle_count {
            for bi in (&ai+1)..particle_count {
                let p1 = ps.particles[ai];
                let p2 = ps.particles[bi];

                // See if two particles will collide. Continue if they do not collide.
                let dist_sqrd = (p1.pos - p2.pos).magnitude2();
                let r12_sqrd = (p1.radius * p1.radius) + (p2.radius * p2.radius);
                if dist_sqrd >= r12_sqrd {
                    continue;
                }


                // https://www.cemyuksel.com/research/papers/particle_merging-and-splitting_tvcg2021.pdf
                // page 3:
                //
                // Consider two particles with masses m1 and m2, posi-
                // tions x1 and x2, and velocities v1 and v2 colliding with
                // each other. We merge the two particles based on an inelastic
                // TRUONG et al.: PARTICLE MERGING-AND-SPLITTING 3
                // collision formulation, such that the total mass, position, and
                // velocity of the meta-particle become
                // m12 = m1 + m2 , (1)
                // x12 = (m1x1 + m2x2) /m12 , (2)
                // v12 = (m1v1 + m2v2) /m12 . (3)
                let m12 = p1.mass + p2.mass;
                let x12 = (p1.mass * p1.pos + p2.mass * p2.pos) / m12;
                let v12 = (p1.mass * p1.vel + p2.mass * p2.vel) / m12;

                let energy_delta = ((p1.mass * p2.mass) / (2.0 * m12)) * (p1.vel - p2.vel).magnitude2();

                let p3 = Particle::default().set_pos(x12).set_vel(v12).set_mass(m12);
                // todo:
            }
        }
    }
}

impl Default for OperationMerge {
    fn default() -> Self {
        Self {
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{math::Vec2, particle_system::ParticleSystem};
    use super::*;

    #[test]
    fn execute() {
        let mut ps = ParticleSystem::default();
        let p1 = *Particle::default().set_vel(Vec2::new(0.1, 0.0));
        let p2 = *Particle::default().set_pos(Vec2::new(1.0, 0.0));

        ps.particles.push(p1);
        ps.particles.push(p2);

        let psm = OperationMerge::default();
        psm.execute(&mut ps);

        //assert_eq!(ps.particles, vec![]);
    }
}