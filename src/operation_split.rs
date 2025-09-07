use cgmath::InnerSpace;

use crate::{operation::Operation, particle::Particle, particle_system::ParticleSystem};



pub struct OperationSplit {
}

impl Operation for OperationSplit {
    fn execute(&self, ps: &mut ParticleSystem) {
        let particle_count: usize = ps.len();
        for ai in 0..particle_count {
            if ps.particles[ai].is_enabled() {
                continue;
            }

            for bi in (&ai+1)..particle_count {
                let p1 = &ps.particles[ai];
                let p2 = &ps.particles[bi];
                if p2.is_enabled() {
                    continue;
                }

                // todo: split the disabled particles.
                // Disabled particles are disabled as they have been "merged" with another.
            }
        }
    }
}

impl Default for OperationSplit {
    fn default() -> Self {
        Self {
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{math::Vec2, operation_merge::OperationMerge, particle_system::ParticleSystem};
    use super::*;

    #[test]
    fn marge_then_split() {
        let mut ps = ParticleSystem::default();
        let p1 = Particle::default();
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0));

        ps.particles.push(p1);
        ps.particles.push(p2);

        // This should merge p2 into p1 as they intersect.
        let psm = OperationMerge::default();
        psm.execute(&mut ps);

        assert_eq!(ps.particles[0].is_enabled(), true);
        assert_eq!(ps.particles[0].merge_index, usize::MAX);

        assert_eq!(ps.particles[1].is_enabled(), false); // Disabled because it has been merged.
        assert_eq!(ps.particles[1].merge_index, 0);

        // todo: split
    }
}