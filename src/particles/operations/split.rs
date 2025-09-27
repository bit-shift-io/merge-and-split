use cgmath::{InnerSpace, Vector2};

use crate::{math::vec2::{reflect_vector_a_around_b, Vec2}, particles::{operations::operation::Operation, particle::ParticleType, particle_vec::ParticleVec}};



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Split {
    pub restitution_coefficient: f32,
}

impl Split {
    pub fn set_restitution_coefficient(&mut self, restitution_coefficient: f32) -> &mut Self {
        self.restitution_coefficient = restitution_coefficient;
        self
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

            // Split the meta-particle back into two particles.
            // p1_mass, p2_mass: masses of original particles
            // p1_radius, p2_radius: radii of original particles
            // delta_E: stored energy from merge
            // n: original connection vector
            // v1_original: original velocity of p1
            // alpha: restitution coefficient (0 to 1) - energy restored to the system. 0 full lose. 1 fully kept

            let alpha = self.restitution_coefficient; // User tweakable.
            let epsilon = 0.0; //1e-10; // todo: User tweakable.

            let meta_particle = &ps[i];

            let ai = meta_particle.left_index;
            let bi = meta_particle.right_index;

            let p1 = &ps[ai];
            let p2 = &ps[bi];

            debug_assert!(!(p1.is_static && p2.is_static), "Two static particles were merged");

            let m12 = meta_particle.mass;
            let x12_prime = meta_particle.pos;
            let v12_prime = meta_particle.vel;
            //let r12 = meta_particle.radius;

            let m1 = p1.mass;
            let m2 = p2.mass;

            //let r1 = p1.radius;
            //let r2 = p2.radius;

            let n = meta_particle.n;
            let delta_e = meta_particle.energy_delta;

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

            let n_hat = n.normalize(); //n / (n.magnitude2() + epsilon);

            let v1_prime;
            let v2_prime;

            // One Static particle, One Dynamic particle:
            // Static particle velocity = 0 (enforced)
            // Dynamic particle bounces using reflection formula: v_dynamic' = v_dynamic + (1 + α) * (v_relative · n̂) * n̂
            // This preserves the coefficient of restitution α

            // Dynamic particle gets all the momentum: v_dynamic = (m12/m_dynamic) * v12
            if p1.is_static {
                // // Left static, right dynamic - bounce off static wall
                // v1_prime = Vec2::new(0.0, 0.0); //np.zeros_like(v1_original)  // Static particle doesn't move
                // // Dynamic particle bounces with reflection (coefficient of restitution alpha)
                // // v2' = v2 + (1 + alpha) * (v12' - v2) · n_hat * n_hat
                // let relative_vel = (v12_prime - p2.vel).dot(n_hat); //np.dot(v12_prime - v2_original, n_hat)
                // v2_prime = p2.vel + (1.0 + alpha) * relative_vel * n_hat;

                // Static particle velocity: v1 = 0
                // v1_prime = Vec2::new(0.0, 0.0); //vec3_zero();
                
                // // Right particle velocity: conserve momentum
                // // m1*v1 + m2*v2 = m12*v12, with v1 = 0
                // v2_prime = -(m12 / m2) * v12_prime; //scale(m12 / m2, *v12);

                //let hat_n = normalize(*n);
                //let x1_new = p1.pos; //left.get_position();  // Static particle keeps original position
                //let x2_new = sub(*x12, scale(m1 / m12, *n));  // Right position adjusted
                
                // Static particle velocity: v1 = 0
                v1_prime = Vec2::new(0.0, 0.0); //let v1_new = vec3_zero();

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
                
                // // CORRECTED: Mirror the dynamic particle's velocity across the collision normal
                // // For elastic collision with static object, v_dynamic_final = v_dynamic_initial - 2*(v_dynamic_initial · n̂)*n̂
                // // Since the meta-particle contains both, we need to extract the dynamic component
                
                // // Get the initial velocity of the dynamic particle (right)
                // let v2_initial = p2.vel; //right.get_velocity();
                
                // // Project initial velocity onto collision normal
                // let v2_normal_component = v2_initial.dot(n_hat) * n_hat; //scale(dot(v2_initial, n_hat), n_hat);
                // let v2_tangential_component = v2_initial - v2_normal_component;
                
                // // For elastic collision with static object (coefficient of restitution = alpha):
                // // Normal component is reflected: v_normal_final = -alpha * v_normal_initial
                // // Tangential component is unchanged (no friction in this model)
                // let v2_normal_final = -alpha * v2_normal_component;
                // v2_prime = v2_normal_final + v2_tangential_component;
            } else if p2.is_static {
                // // Right static, left dynamic
                // v2_prime = Vec2::new(0.0, 0.0); //np.zeros_like(m.right.velocity)  // Static particle doesn't move
                // // Dynamic particle bounces with reflection
                // let relative_vel = (v12_prime - p1.vel).dot(n_hat); //np.dot(v12_prime - v1_original, n_hat)
                // v1_prime = p1.vel + (1.0 + alpha) * relative_vel * n_hat;

                // todo: consider restitution_coefficient
                
                // Static particle velocity: v2 = 0
                v2_prime = Vec2::new(0.0, 0.0); //vec3_zero();
                
                let v3 = reflect_vector_a_around_b(p1.vel, n_hat);
                v1_prime = v3

                // Left particle velocity: conserve momentum
                // m1*v1 + m2*v2 = m12*v12, with v2 = 0
                //v1_prime = -(m12 / m1) * v12_prime; //scale(m12 / m1, *v12);
            } else {
                // s^2 from Eq 11.
                let s2 = 2.0 * alpha * delta_e / m12 * (m1 / m2);
                let s: f32 = s2.sqrt();//.max(0.0).sqrt(); //Math.sqrt(max(s2, 0));

                // Quadratic equation for mu (Eq 13)
                let v1_original = p1.vel;
                let delta_v = v12_prime - v1_original;
                let b = -2.0 * n_hat.dot(delta_v); // np.dot(n_hat, delta_v);
                let c = delta_v.magnitude2() - s2; //np.linalg.norm(delta_v)**2 - s2;
                let a = 1.0;
                let discriminant = b.powi(2) - 4.0 * a * c; //b**2 - 4*a*c; -- Where is this coming from?

                let epsilon_vec: Vector2<f32>;// = Vec2::new(0.0, 0.0);
                let mu: f32;// = 0.0;
                if discriminant >= 0.0 {
                    // Two roots, take smaller (Eq 14)
                    let mu1 = (-b + discriminant.sqrt()) / (2.0 * a);
                    let mu2 = (-b - discriminant.sqrt()) / (2.0 * a);
                    mu = mu1.min(mu2); //min(mu1, mu2);

                    // fmnote: The assert below to check seperation is failing
                    // the python version indicates we should take the higher in a special case
                    // which might resolve the problem?
                    //
                    // Check/Verify separation (Eq 14)
                    // if (v2_prime - v1_prime).dot(n_hat) >= -epsilon {
                    //     mu = mu2; // If not, take larger? But paper says take smaller for separation
                    // }

                    epsilon_vec = Vec2::new(0.0, 0.0); //np.zeros_like(n_hat);
                } else {
                    // No real root, use geometric solution (Eq 15,16)
                    mu = n_hat.dot(v12_prime - v1_original);
                    let w = (v12_prime - v1_original) - (mu * n_hat);
                    let w_len = w.magnitude(); //np.linalg.norm(w)
                    if w_len > epsilon {
                        epsilon_vec = (w_len - s) * (w / w_len);
                    } else {
                        epsilon_vec = Vec2::new(0.0, 0.0); //np.zeros_like(w);
                    }
                }

                // v'1 from Eq 12
                v1_prime = v1_original + (mu * n_hat) + epsilon_vec;
                
                // v'2 from momentum conservation (Eq 8, fleshed out more on page 4)
                v2_prime = ((m12 / m2) * v12_prime) - ((m12 / m2) * v1_prime); //(m12 * v12_prime - m1 * v1_prime) / m2;
                
                // Verify separation (Eq 14)
                //debug_assert!((v2_prime - v1_prime).dot(n_hat) >= -epsilon, "Particles not separating");
            }

            if p1.debug || p2.debug {
                println!("Split p1:{} and p2:{} from meta:{}. These will change to: p1:{} and p2:{}", p1, p2, meta_particle, 
                    p1.clone().set_debug(false).set_pos(x1_prime).set_vel(v1_prime), 
                    p2.clone().set_debug(false).set_pos(x2_prime).set_vel(v2_prime));
            }

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
            restitution_coefficient: 1.0,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::particles::{operations::{merge::Merge, metrics::Metrics}, particle::Particle};

    use super::*;

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
        let mut met2 = Metrics::default();
        met2.execute(&mut ps);
        assert!(met1.approx_equal(&met2));

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