use cgmath::{InnerSpace, Vector2};

use crate::{math::vec2::{reflect_vector_a_around_b, Vec2}, particles::{operations::operation::Operation, particle::{Particle, ParticleType}, particle_vec::ParticleVec}};



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Split {
    pub restitution_coefficient: f32,
}

impl Split {
    pub fn set_restitution_coefficient(&mut self, restitution_coefficient: f32) -> &mut Self {
        self.restitution_coefficient = restitution_coefficient;
        self
    }

    pub fn split_meta_particle(&self, meta_particle: &Particle, p1: &Particle, p2: &Particle) -> (Particle, Particle, Particle) {
        debug_assert!(meta_particle.particle_type == ParticleType::MetaParticle);

        // Split the meta-particle back into two particles.
        // p1_mass, p2_mass: masses of original particles
        // p1_radius, p2_radius: radii of original particles
        // delta_E: stored energy from merge
        // n: original connection vector
        // v1_original: original velocity of p1
        // alpha: restitution coefficient (0 to 1) - energy restored to the system. 0 full lose. 1 fully kept

        let alpha = self.restitution_coefficient; // User tweakable.
        let epsilon = 0.0; //1e-10; // todo: User tweakable.

        debug_assert!(!(p1.is_static && p2.is_static), "Two static particles were merged");

        let m12 = meta_particle.mass;
        let x12_prime = meta_particle.pos;
        let v12_prime = meta_particle.vel;

        let m1 = p1.mass;
        let m2 = p2.mass;

        let n = meta_particle.n;
        let delta_e = meta_particle.energy_delta;
        debug_assert!(delta_e >= 0.0, "Negative energy not possible");

        // Compute positions of split particles (Eq 6,7)
        let mut x1_prime = x12_prime - (m2 / m12) * n;
        let mut x2_prime = x12_prime + (m1 / m12) * n;

        if p1.is_static {
            x1_prime = p1.pos;
            //x2_prime = x12_prime - (m1 / m12) * n; //sub(*x12, scale(m1 / m12, *n));  // Right position adjusted
        }
        if p2.is_static {
            //x1_prime = x12_prime + (m2 / m12) * n; //add(*x12, scale(m2 / m12, *n));  // Left position adjusted
            x2_prime = p2.pos;
        }

        let n_hat = n.normalize();

        let v1_prime;
        let v2_prime;

        // One Static particle, One Dynamic particle:
        // Static particle velocity = 0 (enforced)
        // Dynamic particle bounces using reflection formula: v_dynamic' = v_dynamic + (1 + α) * (v_relative · n̂) * n̂
        // This preserves the coefficient of restitution α

        // Dynamic particle gets all the momentum: v_dynamic = (m12/m_dynamic) * v12
        if p1.is_static {
            
            // Static particle velocity: v1 = 0
            v1_prime = Vec2::new(0.0, 0.0); 

            // To reflect a vector a off a tangent surface in cgmath, 
            // you should first find the tangent vector and the normal vector of the surface, 
            // then use the formula 
            // R = a - 2 * dot(a, N) * N, 
            // where N is the unit normal vector and 
            // R is the reflected vector. 
            // This formula computes the reflection of vector a about a surface defined by its normal N

            // todo: consider restitution_coefficient
            let v2 = p2.vel - 2.0 * p2.vel.dot(n_hat) * n_hat;
            let v3 = reflect_vector_a_around_b(p2.vel, n_hat);
            v2_prime = v3
        } else if p2.is_static {
            // todo: consider restitution_coefficient
            
            // Static particle velocity: v2 = 0
            v2_prime = Vec2::new(0.0, 0.0);
            
            let v3 = reflect_vector_a_around_b(p1.vel, n_hat);
            v1_prime = v3
        } else {
            // s^2 from Eq 11. on page 3
            let s2 = (2.0 * alpha * delta_e) / (m12 * (m2 / m1));
            let s: f32 = s2.sqrt();//.max(0.0).sqrt(); //Math.sqrt(max(s2, 0));

            // Now we are attempting to find a solution for v1_prime as the paper says:
            // "Hence the only unknown if v1_prime, and once it is solved, 
            // v2_prime can be calculated using momentum conservation (Eq 8)"

            // Solve Quadratic equation for mu (Eq 13)
            // ax² + bx + c = 0
            let v1_original = p1.vel;
            let delta_v = v12_prime - v1_original;

            let a = 1.0; // mu ^ 2 in Eq 13. why is this 1?
            let b = -2.0 * n_hat.dot(delta_v);
            let c = delta_v.magnitude2() - s2;
            let discriminant = b * b - 4.0 * a * c;

            let epsilon_vec: Vector2<f32>;
            let mu: f32;
            if discriminant >= 0.0 {
                // Calculate roots using quadratic formula
                let sqrt_d = discriminant.sqrt();
                let two_a = 2.0 * a;
                let root1 = (-b + sqrt_d) / two_a;
                let root2 = (-b - sqrt_d) / two_a;

                // Two roots, take smaller (Eq 14)
                // if root1.abs() < root2.abs() {
                //     mu = root1;
                // } else {
                //     mu = root2;
                // }
                mu = root1.min(root2); // this typically gives a -ve number which we need for this to work. The above code I commented out does not do this.

                epsilon_vec = Vec2::new(0.0, 0.0);
            } else {
                // No real root, use geometric solution (Eq 15,16)
                mu = n_hat.dot(delta_v);
                let w = delta_v - (mu * n_hat);
                let w_len = w.magnitude();
                if w_len > epsilon {
                    epsilon_vec = (w_len - s) * (w / w_len);
                } else {
                    epsilon_vec = Vec2::new(0.0, 0.0);
                }
            }

            // FM: I think there is a problem here. Velocity is not changing direction
            // in the unit test, so the 2 particles keep moving towards each other.
            // v'1 from Eq 12
            v1_prime = v1_original + (mu * n_hat) + epsilon_vec;
            
            // v'2 from momentum conservation (Eq 8, fleshed out more on page 4)
            v2_prime = ((m12 / m2) * v12_prime) - ((m1 / m2) * v1_prime);
            
            // Verify conservation of momentum
            // #[cfg(debug_assertions)]
            // {
            //     use core::f32;
            //     use crate::math::float::float_approx_equal;

            //     if self.restitution_coefficient == 1.0 {
            //         let p1_momentum = (p1.mass * p1.vel).magnitude();
            //         let p2_momentum = (p2.mass * p2.vel).magnitude();
            //         let moment_before = p1_momentum + p2_momentum;

            //         let p1_momentum_prime = (p1.mass * v1_prime).magnitude();
            //         let p2_momentum_prime = (p2.mass * v2_prime).magnitude();
            //         let momentum_after = p1_momentum_prime + p2_momentum_prime;
            //         debug_assert!(float_approx_equal(moment_before, momentum_after, f32::EPSILON) , "Momentum not conserved");
            //     }
            // }

            // Verify separation (Eq 14)
            #[cfg(debug_assertions)]
            {
                // If there are 2 real roots, particle seperation is guarnateed.
                let delta_v = v2_prime - v1_prime;
                let d = delta_v.dot(n_hat);
                debug_assert!(d >= 0.0, "Particles not separating");
            }
        }

        if p1.debug || p2.debug {
            println!("Split p1:{} and p2:{} from meta:{}. These will change to: p1:{} and p2:{}", p1, p2, meta_particle, 
                p1.clone().set_debug(false).set_pos(x1_prime).set_vel(v1_prime), 
                p2.clone().set_debug(false).set_pos(x2_prime).set_vel(v2_prime));
        }

        let meta_particle_prime = *meta_particle.clone().set_merged(false);

        let p1_prime = *p1.clone()
            .set_merged(false)
            .set_pos(x1_prime)
            .set_vel(v1_prime);

        let p2_prime = *p2.clone()
            .set_merged(false)
            .set_pos(x2_prime)
            .set_vel(v2_prime);
        
        (meta_particle_prime, p1_prime, p2_prime)
    }
}

impl Operation for Split {
    fn execute(&mut self, ps: &mut ParticleVec) {
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

            let (meta_particle_prime, p1_prime, p2_prime) = {
                let meta_particle = &ps[i];
                let p1 = &ps[meta_particle.left_index];
                let p2 = &ps[meta_particle.right_index];
                self.split_meta_particle(meta_particle, p1, p2)
            };

            {
                ps[meta_particle_prime.left_index] = p1_prime;
                ps[meta_particle_prime.right_index] = p2_prime;
            }
        }

        // Remove Meta Particles
        ps.truncate(first_meta_particle_index);
    }
}

impl Default for Split {
    fn default() -> Self {
        Self {
            restitution_coefficient: 1.0,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::particles::{operations::{merge::Merge, metrics::Metrics}, particle::Particle};

    use super::*;

    #[test]
    fn merge_and_split_3_intersecting_order_independent() {
        let p1 = *Particle::default().set_debug(true).set_pos(Vec2::new(0.0, 0.0)).set_vel(Vec2::new(0.1, 0.0)); // At origin.
        let p2 = *Particle::default().set_debug(true).set_pos(Vec2::new(0.9, 0.0)); // To the right of p1 such that it just overlaps.
        let p3 = *Particle::default().set_debug(true).set_pos(Vec2::new(0.5, 0.5)); // Between p1 and p2, but higher, so all 3 overlap.

        // Changing particle order should not change things.
        let mut ps1 = ParticleVec::from([p1, p2, p3]);
        let mut ps2 = ParticleVec::from([p2, p1, p3]);

        let col1 = Merge::default().compute_collisions(&ps1);
        let col2 = Merge::default().compute_collisions(&ps2);

        // Measure metrics
        let mut met1 = Metrics::default();
        met1.execute(&mut ps1);

        // This should merge p2 and p1 as they intersect.
        let mut psm = Merge::default();
        println!("Merging ps1 - p1, p2, p3");
        psm.execute(&mut ps1);

        println!("Merging ps2 - p2, p1, p3");
        psm.execute(&mut ps2);

        assert_eq!(ps1.len(), 5); // Two meta particles were created.
        assert_eq!(ps2.len(), 5); // Two meta particles were created.

        assert_eq!(ps1[4], ps2[4]); // The top level meta particles should be the same regardless of order.
    }


    #[test]
    fn merge_and_split_2_intersecting() {
        let p1 = *Particle::default().set_vel(Vec2::new(0.1, 0.0));
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0));

        let mut ps = ParticleVec::from([p1, p2]);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false);

        // Measure metrics
        let mut met1 = Metrics::default();
        met1.execute(&mut ps);

        // This should merge p2 and p1 as they intersect.
        let mut psm = Merge::default();
        psm.execute(&mut ps);

        assert_eq!(ps.len(), 3); // A meta particle has been added to the Particle System.

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, true);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, true);

        assert_eq!(ps[2].particle_type, ParticleType::MetaParticle);
        assert_eq!(ps[2].is_merged, false);

        // This should split the meta particle.
        let mut pss = Split::default().set_restitution_coefficient(1.0).clone();
        pss.execute(&mut ps);

        // Measure metrics
        let mut met2 = Metrics::default();
        met2.execute(&mut ps);
        assert!(met1.approx_equal(&met2));

        assert_eq!(ps.len(), 2);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false);  

        // Measure metrics again to see if there is any change
        let mut met3 = Metrics::default();
        met3.execute(&mut ps);
        assert!(met1.approx_equal(&met3));       
    }


    #[test]
    fn merge_and_split_3_intersecting() {
        let p1 = *Particle::default().set_pos(Vec2::new(0.0, 0.0)).set_vel(Vec2::new(0.1, 0.0)); // At origin.
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0)); // To the right of p1 such that it just overlaps.
        let p3 = *Particle::default().set_pos(Vec2::new(0.5, 0.5)); // Between p1 and p2, but higher, so all 3 overlap.

        let mut ps = ParticleVec::from([p1, p2, p3]);
        
        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false);

        assert_eq!(ps[2].particle_type, ParticleType::Particle);
        assert_eq!(ps[2].is_merged, false);

        // Measure metrics
        let mut met1 = Metrics::default();
        met1.execute(&mut ps);

        // This should merge p1, p2 and p3 as they intersect.
        let mut psm = Merge::default();
        psm.execute(&mut ps);

        // Measure metrics
        // let mut met2 = Metrics::default();
        // met2.execute(&mut ps);
        // assert!(met1.approx_equal(&met2));

        assert_eq!(ps.len(), 5); // 3 original particles + 2 meta particle. 2 meta particles have been added to the Particle System.

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, true);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, true);

        assert_eq!(ps[2].particle_type, ParticleType::Particle);
        assert_eq!(ps[2].is_merged, true);

        assert_eq!(ps[3].particle_type, ParticleType::MetaParticle); // The merging of p1 and p2 -> p12
        assert_eq!(ps[3].is_merged, true);
        assert_eq!(ps[3].left_index, 0);
        assert_eq!(ps[3].right_index, 1);

        assert_eq!(ps[4].particle_type, ParticleType::MetaParticle); // The merging of p12 and p3 -> p123
        assert_eq!(ps[4].is_merged, false);
        assert_eq!(ps[4].left_index, 3);
        assert_eq!(ps[4].right_index, 2);

        // This should split the meta particles.
        let mut pss = Split::default();
        pss.execute(&mut ps);

        assert_eq!(ps.len(), 3);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false);

        assert_eq!(ps[2].particle_type, ParticleType::Particle);
        assert_eq!(ps[2].is_merged, false);

        // Measure metrics again to see if there is any change
        let mut met3 = Metrics::default();
        met3.execute(&mut ps);
        assert!(met1.approx_equal(&met3));
    }


    #[test]
    fn merge_and_split_2_static_intersecting() {
        let p1 = *Particle::default().set_pos(Vec2::new(0.0, 0.0)).set_static(true);
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0)).set_vel(Vec2::new(-0.1, 0.0));

        let mut ps = ParticleVec::from([p1, p2]);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);
        assert_eq!(ps[0].is_static, true);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false);
        assert_eq!(ps[1].is_static, false);

        // This should merge p2 and p1 as they intersect.
        let mut psm = Merge::default();
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
        let mut pss = Split::default();
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
        //assert_eq!(ps[1].pos, Vec2::new(0.9, 0.0));
        assert_eq!(ps[1].vel, Vec2::new(0.1, 0.0));   
    }

}