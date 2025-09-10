use cgmath::InnerSpace;

use crate::{math::Vec2, operation::Operation, particle::{Particle, ParticleType}, particle_system::ParticleSystem};



pub struct OperationSplit {
}

impl Operation for OperationSplit {
    fn execute(&self, ps: &mut ParticleSystem) {
        let particle_count: usize = ps.len();

        // Meta Particles are always at the end of the Particle System.
        // We are only interested in splitting MetaParticles.
        let mut first_meta_particle_index = 0;
        for i in 0..particle_count {
            // We are only interested in splitting MetaParticles.
            if ps.particles[i].particle_type != ParticleType::MetaParticle {
                continue;
            }

            first_meta_particle_index = i;
            break;
        }

        for i in first_meta_particle_index..particle_count {
            debug_assert!(ps.particles[i].particle_type == ParticleType::MetaParticle);

            // Split the meta-particle back into two particles.
            // p1_mass, p2_mass: masses of original particles
            // p1_radius, p2_radius: radii of original particles
            // delta_E: stored energy from merge
            // n: original connection vector
            // v1_original: original velocity of p1
            // alpha: restitution coefficient (0 to 1)

            let alpha = 1.0; // todo: User tweakable.
            let epsilon = 1e-10; // todo: User tweakable.

            let meta_particle = &ps.particles[i];

            let ai = meta_particle.left_index;
            let bi = meta_particle.right_index;

            let p1 = &ps.particles[ai];
            let p2 = &ps.particles[bi];

            let m12 = meta_particle.mass;
            let x12_prime = meta_particle.pos;
            let v12_prime = meta_particle.vel;
            let r12 = meta_particle.radius;

            let m1 = p1.mass;
            let m2 = p2.mass;

            let r1 = p1.radius;
            let r2 = p2.radius;

            let n = meta_particle.n;
            let delta_e = meta_particle.energy_delta;

            // Compute positions of split particles (Eq 6,7)
            let x1_prime = x12_prime - (m2 / m12) * n;
            let x2_prime = x12_prime + (m1 / m12) * n;

            let n_hat = n / (n.magnitude2() + epsilon);

            // s^2 from Eq 11
            let s2 = 2.0 * alpha * delta_e / m12 * (m1 / m2);
            let s = s2.max(0.0).sqrt(); //Math.sqrt(max(s2, 0));

            // Quadratic equation for mu (Eq 13)
            let v1_original = p1.vel;
            let delta_v = v12_prime - v1_original;
            let b = -2.0 * n_hat.dot(delta_v); // np.dot(n_hat, delta_v);
            let c = delta_v.magnitude2() - s2; //np.linalg.norm(delta_v)**2 - s2;
            let a = 1.0;
            let discriminant = b.powi(2) - 4.0 * a * c; //b**2 - 4*a*c;

            let mut epsilon_vec = Vec2::new(0.0, 0.0);
            let mut mu = 0.0;
            if discriminant >= 0.0 {
                // Two roots, take smaller (Eq 14)
                let mu1 = (-b + discriminant.sqrt()) / (2.0 * a);
                let mu2 = (-b - discriminant.sqrt()) / (2.0 * a);
                mu = mu1.min(mu2); //min(mu1, mu2);
                epsilon_vec = Vec2::new(0.0, 0.0); //np.zeros_like(n_hat);
            } else {
                // No real root, use geometric solution (Eq 15,16)
                mu = n_hat.dot(v12_prime - v1_original);
                let w = v12_prime - v1_original - mu * n_hat;
                let w_norm = w.magnitude(); //np.linalg.norm(w)
                if w_norm > epsilon {
                    epsilon_vec = (w_norm - s) * w / w_norm;
                } else {
                    epsilon_vec = Vec2::new(0.0, 0.0); //np.zeros_like(w);
                }
            }

            // v'1 from Eq 12
            let v1_prime = v1_original + mu * n_hat + epsilon_vec;
            
            // v'2 from momentum conservation (Eq 8)
            let v2_prime = (m12 * v12_prime - m1 * v1_prime) / m2;
            
            // Verify separation (Eq 14)
            debug_assert!((v2_prime - v1_prime).dot(n_hat) >= -epsilon, "Particles not separating");


            {
                let p1_mut = &mut ps.particles[ai];
                p1_mut.set_particle_type(ParticleType::Particle)
                    .set_pos(x1_prime)
                    .set_vel(v1_prime);
            }

            {
                let p2_mut = &mut ps.particles[bi];
                p2_mut.set_particle_type(ParticleType::Particle)
                    .set_pos(x2_prime)
                    .set_vel(v2_prime);
            }
        }

        // Remove Meta Particles
        ps.particles.truncate(first_meta_particle_index);
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
        let p1 = *Particle::default().set_vel(Vec2::new(0.1, 0.0));
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0));

        ps.particles.push(p1);
        ps.particles.push(p2);

        assert_eq!(ps.particles[0].particle_type, ParticleType::Particle);
        assert_eq!(ps.particles[1].particle_type, ParticleType::Particle);

        // This should merge p2 and p1 as they intersect.
        let psm = OperationMerge::default();
        psm.execute(&mut ps);

        assert_eq!(ps.particles[0].particle_type, ParticleType::MergedParticle);
        assert_eq!(ps.particles[1].particle_type, ParticleType::MergedParticle);
        assert_eq!(ps.len(), 3); // A meta particle has been added to the Particle System.

        assert_eq!(ps.particles[2].particle_type, ParticleType::MetaParticle);

        // This should split particle.
        let pss = OperationSplit::default();
        pss.execute(&mut ps);

        assert_eq!(ps.particles[0].particle_type, ParticleType::Particle);
        assert_eq!(ps.particles[1].particle_type, ParticleType::Particle);
        assert_eq!(ps.len(), 2);
    }
}