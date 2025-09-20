use cgmath::{InnerSpace, Vector2};

use crate::{math::vec2::Vec2, particles::{operations::operation::Operation, particle::ParticleType, particle_vec::ParticleVec}};




pub struct Split {
}

impl Operation for Split {
    fn execute(&self, ps: &mut ParticleVec) {
        let particle_count: usize = ps.len();

        // The more to the right of the particle system vector we go, 
        // the more recursion depth of merged meta particles there are.
        // So lets work backwards and split the meta partcies. By the time we get back to the first particle all
        // meta particles will be split back into their origional particles.

        // Meta Particles are always at the end of the Particle System.
        // We are only interested in splitting MetaParticles.
        // to think about: Keep meta particles in a seperate list?
        let mut first_meta_particle_index = usize::MAX;
        for i in 0..particle_count {
            // We are only interested in splitting MetaParticles.
            if ps[i].particle_type != ParticleType::MetaParticle {
                continue;
            }

            first_meta_particle_index = i;
            break;
        }

        if first_meta_particle_index == usize::MAX {
            // No MetaParticles to split.
            return;
        }

        // Here we start iterating backwards backwards.
        for i in (first_meta_particle_index..particle_count).rev() {
            debug_assert!(ps[i].particle_type == ParticleType::MetaParticle);

            // Split the meta-particle back into two particles.
            // p1_mass, p2_mass: masses of original particles
            // p1_radius, p2_radius: radii of original particles
            // delta_E: stored energy from merge
            // n: original connection vector
            // v1_original: original velocity of p1
            // alpha: restitution coefficient (0 to 1)

            let alpha = 1.0; // todo: User tweakable.
            let epsilon = 1e-10; // todo: User tweakable.

            let meta_particle = &ps[i];

            let ai = meta_particle.left_index;
            let bi = meta_particle.right_index;

            let p1 = &ps[ai];
            let p2 = &ps[bi];

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

            let epsilon_vec: Vector2<f32>;// = Vec2::new(0.0, 0.0);
            let mu: f32;// = 0.0;
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
            
            debug_assert!(!(p1.is_static && p2.is_static), "Two static particles were maerged");

            // Test case - p1 = static, p2 = dynamic
            if p1.is_static {
                {
                    let p1_mut = &mut ps[ai];
                    p1_mut.set_merged(false);
                }

                {
                    let combined_v12_prime = v2_prime - v1_prime;
                    let combined_x12_prime = x2_prime;

                    let p2_mut = &mut ps[bi];
                    p2_mut.set_merged(false)
                        .set_pos(combined_x12_prime)
                        .set_vel(combined_v12_prime);
                }
                continue;
            }

            // Test case - p1 = dynamic, p2 = static
            if p2.is_static {
                {
                    let combined_v12_prime = v2_prime - v1_prime;
                    let combined_x12_prime = x2_prime;

                    let p1_mut = &mut ps[ai];
                    p1_mut.set_merged(false)
                        .set_pos(combined_x12_prime)
                        .set_vel(combined_v12_prime);
                }

                {
                    let p2_mut = &mut ps[bi];
                    p2_mut.set_merged(false);
                }
                continue;
            }

            // Verify separation (Eq 14) - throwing in static particles breaks this assert.
            //debug_assert!((v2_prime - v1_prime).dot(n_hat) >= -epsilon, "Particles not separating");

            {
                let p1_mut = &mut ps[ai];
                p1_mut.set_merged(false)
                    .set_pos(x1_prime)
                    .set_vel(v1_prime);
            }

            {
                let p2_mut = &mut ps[bi];
                p2_mut.set_merged(false)
                    .set_pos(x2_prime)
                    .set_vel(v2_prime);
            }
        }

        // Remove Meta Particles
        ps.truncate(first_meta_particle_index);
    }
}

impl Default for Split {
    fn default() -> Self {
        Self {
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::particles::{operations::merge::Merge, particle::Particle};

    use super::*;


    #[test]
    fn merge_and_split_2_intersecting() {
        let mut ps = ParticleVec::default();
        let p1 = *Particle::default().set_vel(Vec2::new(0.1, 0.0));
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0));

        ps.push(p1);
        ps.push(p2);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false);

        // This should merge p2 and p1 as they intersect.
        let psm = Merge::default();
        psm.execute(&mut ps);

        assert_eq!(ps.len(), 3); // A meta particle has been added to the Particle System.

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, true);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, true);

        assert_eq!(ps[2].particle_type, ParticleType::MetaParticle);
        assert_eq!(ps[2].is_merged, false);

        // This should split the meta particle.
        let pss = Split::default();
        pss.execute(&mut ps);

        assert_eq!(ps.len(), 2);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false);        
    }


    #[test]
    fn merge_and_split_3_intersecting() {
        let mut ps = ParticleVec::default();
        let p1 = *Particle::default().set_pos(Vec2::new(0.0, 0.0)).set_vel(Vec2::new(0.1, 0.0)); // At origin.
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0)); // To the right of p1 such that it just overlaps.
        let p3 = *Particle::default().set_pos(Vec2::new(0.5, 0.5)); // Between p1 and p2, but higher, so all 3 overlap.

        ps.push(p1);
        ps.push(p2);
        ps.push(p3);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false);

        assert_eq!(ps[2].particle_type, ParticleType::Particle);
        assert_eq!(ps[2].is_merged, false);

        // This should merge p1, p2 and p3 as they intersect.
        let psm = Merge::default();
        psm.execute(&mut ps);

        assert_eq!(ps.len(), 5); // 3 original particles + 2 meta particle. 2 meta particles have been added to the Particle System.

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, true);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, true);

        assert_eq!(ps[2].particle_type, ParticleType::Particle);
        assert_eq!(ps[2].is_merged, true);

        assert_eq!(ps[3].particle_type, ParticleType::MetaParticle); // The merging of p1 and p2 -> p12
        assert_eq!(ps[3].is_merged, true);

        assert_eq!(ps[4].particle_type, ParticleType::MetaParticle); // The merging of p12 and p3 -> p123
        assert_eq!(ps[4].is_merged, false);

        // This should split the meta particle.
        let pss = Split::default();
        pss.execute(&mut ps);

        assert_eq!(ps.len(), 3);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false);

        assert_eq!(ps[2].particle_type, ParticleType::Particle);
        assert_eq!(ps[2].is_merged, false);
    }


    #[test]
    fn merge_and_split_2_static_intersecting() {
        let mut ps = ParticleVec::default();
        let p1 = *Particle::default().set_pos(Vec2::new(0.0, 0.0)).set_static(true);
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0)).set_vel(Vec2::new(-0.1, 0.0));

        ps.push(p1);
        ps.push(p2);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);
        assert_eq!(ps[0].is_static, true);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false);
        assert_eq!(ps[1].is_static, false);

        // This should merge p2 and p1 as they intersect.
        let psm = Merge::default();
        psm.execute(&mut ps);

        assert_eq!(ps.len(), 3); // A meta particle has been added to the Particle System.

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, true);
        assert_eq!(ps[0].is_static, true);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, true);
        assert_eq!(ps[1].is_static, false);

        assert_eq!(ps[2].particle_type, ParticleType::MetaParticle);
        assert_eq!(ps[2].is_merged, false);

        // This should split the meta particle.
        let pss = Split::default();
        pss.execute(&mut ps);

        assert_eq!(ps.len(), 2);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);
        assert_eq!(ps[0].is_static, true);
        assert_eq!(ps[0].pos, Vec2::new(0.0, 0.0));
        assert_eq!(ps[0].vel, Vec2::new(0.0, 0.0));

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false); 
        assert_eq!(ps[1].is_static, false);       
    }


}