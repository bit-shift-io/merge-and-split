use std::usize;

use cgmath::InnerSpace;

use crate::{math::vec2::Vec2, particles::{operations::operation::Operation, particle::{Particle, ParticleType}, particle_vec::ParticleVec}};


// MetaParticle enum for tree structure
enum MetaParticle {
    Leaf {
        //index: usize,
        particle: Particle,
    },
    Node {
        left: Box<MetaParticle>,
        right: Box<MetaParticle>,
        mass: f32,
        position: Vec2,
        velocity: Vec2,
        delta_e: f32,
        n: Vec2,
        v_left_initial: Vec2,
        v_right_initial: Vec2,
    },
}

impl MetaParticle {
    // Get mass of this meta
    fn get_mass(&self) -> f32 {
        match self {
            MetaParticle::Leaf { particle } => particle.mass, //PARTICLES[*index].mass, // Assume global PARTICLES vec
            MetaParticle::Node { mass, .. } => *mass,
        }
    }

    // Get position (for leaf, from particle; for node, stored)
    fn get_pos(&self) -> Vec2 {
        match self {
            MetaParticle::Leaf { particle } => particle.pos, //PARTICLES[*index].position,
            MetaParticle::Node { position, .. } => *position,
        }
    }

    // Get velocity
    fn get_vel(&self) -> Vec2 {
        match self {
            MetaParticle::Leaf { particle } => particle.vel, //PARTICLES[*index].velocity,
            MetaParticle::Node { velocity, .. } => *velocity,
        }
    }

    // Set position (for node only, leaves updated later)
    fn set_pos(&mut self, pos: Vec2) {
        if let MetaParticle::Node { position, .. } = self {
            *position = pos;
        }
    }

    // Set velocity (for node only)
    fn set_vel(&mut self, vel: Vec2) {
        if let MetaParticle::Node { velocity, .. } = self {
            *velocity = vel;
        }
    }
}

// Function to merge two metas into a node
fn merge(left: MetaParticle, right: MetaParticle) -> MetaParticle {
    let m1 = left.get_mass();
    let m2 = right.get_mass();
    let m12 = m1 + m2;

    let x1 = left.get_pos();
    let x2 = right.get_pos();
    let x12 = ((m1 / m12) * x1) + ((m2 / m12) * x2); // let x12 = add(scale(m1 / m12, x1), scale(m2 / m12, x2));

    let v1 = left.get_vel();
    let v2 = right.get_vel();
    let v12 = ((m1 / m12) * v1) + ((m2 / m12) * v2); //let v12 = add(scale(m1 / m12, v1), scale(m2 / m12, v2));

    let delta_v = v1 - v2; //sub(v1, v2);
    let delta_e = (m1 * m2 / (2.0 * m12)) * delta_v.magnitude2(); //norm_sq(delta_v);

    let n = x2 - x1; //sub(x2, x1);

    MetaParticle::Node {
        left: Box::new(left),
        right: Box::new(right),
        mass: m12,
        position: x12,
        velocity: v12,
        delta_e,
        n,
        v_left_initial: v1,
        v_right_initial: v2,
    }
}

// Recursive function to build meta tree from list of particle indices
fn build_meta_tree(indices: &[usize], ps: &ParticleVec) -> MetaParticle {
    debug_assert!(indices.len() > 0);

    if indices.len() == 1 {
        MetaParticle::Leaf { particle: ps[indices[0]] }
    } else if indices.len() == 2 {
        let left = MetaParticle::Leaf { particle: ps[indices[0]] }; //index: indices[0] };
        let right = MetaParticle::Leaf { particle: ps[indices[1]] }; //index: indices[1] };
        merge(left, right)
    } else {
        // Split into two halves for balanced tree
        let mid = indices.len() / 2;
        let left_tree = build_meta_tree(&indices[0..mid], ps);
        let right_tree = build_meta_tree(&indices[mid..], ps);
        merge(left_tree, right_tree)
    }
}

pub struct Merge {
}

impl Merge {
    pub fn compute_collisions(&self, ps: &ParticleVec) -> Vec<Vec<usize>> {
        //let mut collisions = Vec::with_capacity(ps.len());

        // start off colliding with "self", such that all particles are converted to a metaparticle.
        let mut collisions: Vec<Vec<usize>> = (0..ps.len()).map(|i| vec![i]).collect();

        let particle_count: usize = ps.len();
        for ai in 0..particle_count {
            for bi in (&ai+1)..particle_count {
                let p1 = &ps[ai];
                let p2 = &ps[bi];

                // Two static particles cannot merge.
                if p1.is_static && p2.is_static {
                    continue;
                }
                
                // Collision Rule 1: |x2 − x1| < r1 + r2 (page 5).
                // See if two particles will collide. Continue if they do not collide.
                let dist_sqrd = (p1.pos - p2.pos).magnitude2();
                let r1_plus_r2 = p1.radius + p2.radius;
                let r12_sqrd = r1_plus_r2 * r1_plus_r2;
                if dist_sqrd >= r12_sqrd {
                    continue;
                }

                // Collision Rule 2: n · (v2 − v1) < 0 (page 5).
                let n = p2.pos - p1.pos;
                let rel_v = p2.vel - p1.vel;
                let d = n.dot(rel_v);
                if d >= 0.0 {
                    continue;
                }

                collisions[ai].push(bi);
            }
        }
        collisions
    }

    fn execute2(&mut self, ps: &mut ParticleVec) -> Vec<MetaParticle> {
        let mut meta_particles = vec![];
        let collisions = self.compute_collisions(ps);
        for ci in 0..collisions.len() {
            let particle_collisions = &collisions[ci];

            let meta_particle = build_meta_tree(particle_collisions, ps);
            meta_particles.push(meta_particle);
            // for pci in 0..particle_collisions.len() {
            //     let p1_idx = ci;
            //     let p2_idx = particle_collisions[pci];
            // }
        }

        meta_particles
    }
}

impl Operation for Merge {
    fn execute(&mut self, ps: &mut ParticleVec) {
        let particle_count: usize = ps.len();
        for ai in 0..particle_count {
            // Skip "merged" particles, they are handled by the meta particle.
            if ps[ai].is_merged {
                continue;
            }

            // 'actual_particle_index' tracks which was the most top-level meta particle we are dealing with.
            // When A collide with B, we make X
            // When A collides with C, we substitute X for A then merge X with C, making Y.
            // When A collides with D, we substitute Y and merge with D making Z.
            let mut ai_actual = ai;

            for bi in (&ai+1)..particle_count {
                // Skip "merged" particles, they are handled by the meta particle.
                if ps[bi].is_merged {
                    continue;
                }

                let p1 = &ps[ai];
                let p2 = &ps[bi];

                // Two static particles cannot merge.
                if p1.is_static && p2.is_static {
                    continue;
                }
                
                // Collision Rule 1: |x2 − x1| < r1 + r2 (page 5).
                // See if two particles will collide. Continue if they do not collide.
                let dist_sqrd = (p1.pos - p2.pos).magnitude2();
                let r1_plus_r2 = p1.radius + p2.radius;
                let r12_sqrd = r1_plus_r2 * r1_plus_r2;
                if dist_sqrd >= r12_sqrd {
                    continue;
                }

                // Collision Rule 2: n · (v2 − v1) < 0 (page 5).
                let n = p2.pos - p1.pos;
                let rel_v = p2.vel - p1.vel;
                let d = n.dot(rel_v);
                if d >= 0.0 {
                    continue;
                }

                if p1.debug || p2.debug {
                    println!("Merge p1:{} with p2:{}", p1, p2);
                }

                if ps.len() == 80 {
                    println!("Merge is about to make our bogus P80 particle if frame=151...");
                }

                // This is where we substitute any top-level meta particle for p1.
                let p1_actual = &ps[ai_actual];

                // Merge p1_actual and p2.
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

                let m12 = p1_actual.mass + p2.mass;
                let x12 = (p1_actual.mass * p1_actual.pos + p2.mass * p2.pos) / m12;
                let v12 = (p1_actual.mass * p1_actual.vel + p2.mass * p2.vel) / m12;

                let energy_delta = ((p1_actual.mass * p2.mass) / (2.0 * m12)) * (p1_actual.vel - p2.vel).magnitude2();

                {
                    let p1_mut = &mut ps[ai_actual];
                    p1_mut.set_merged(true);
                }

                {
                    let p2_mut = &mut ps[bi];
                    p2_mut.set_merged(true);
                }

                {
                    // MetaParticles do NOT use radius.
                    let meta_particle = *Particle::default()
                        .set_particle_type(ParticleType::MetaParticle)
                        .set_pos(x12)
                        .set_vel(v12)
                        .set_mass(m12)
                        .set_energy_delta(energy_delta)
                        .set_n(n)
                        .set_left_index(ai_actual)
                        .set_right_index(bi);

                    ai_actual = ps.len();
                    ps.push(meta_particle);
                }
            }
        }
    }
}

impl Default for Merge {
    fn default() -> Self {
        Self {
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::math::vec2::Vec2;

    use super::*;

    #[test]
    fn execute2_merge_3_intersecting() {
        let p1 = *Particle::default().set_pos(Vec2::new(0.0, 0.0)).set_vel(Vec2::new(0.1, 0.0)); // At origin.
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0)); // To the right of p1 such that it just overlaps.
        let p3 = *Particle::default().set_pos(Vec2::new(0.5, 0.5)); // Between p1 and p2, but higher, so all 3 overlap.

        let mut ps = ParticleVec::from([p1, p2, p3]);

        let mut m = Merge::default();
        let meta_particles = m.execute2(&mut ps);
        println!("done")
    }

    #[test]
    fn merge_2_intersecting() {
        let p1 = *Particle::default().set_vel(Vec2::new(0.1, 0.0));
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0));

        let mut ps = ParticleVec::from([p1, p2]);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[0].is_merged, false);

        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].is_merged, false);

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
    }

    #[test]
    fn merge_3_intersecting() {
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

        // This should merge p1, p2 and p3 as they intersect.
        let mut psm = Merge::default();
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
        assert_eq!(ps[3].left_index, 0);
        assert_eq!(ps[3].right_index, 1);

        assert_eq!(ps[4].particle_type, ParticleType::MetaParticle); // The merging of p12 and p3 -> p123
        assert_eq!(ps[4].is_merged, false);
        assert_eq!(ps[4].left_index, 3);
        assert_eq!(ps[4].right_index, 2);
    }

    #[test]
    fn ignore_non_intersecting() {
        let p1 = Particle::default();
        let p2 = *Particle::default().set_pos(Vec2::new(1.1, 0.0));

        let mut ps = ParticleVec::from([p1, p2]);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].particle_type, ParticleType::Particle);

        // This should NOT merge p1 and p2, as they are not close enough.
        let mut psm = Merge::default();
        psm.execute(&mut ps);

        assert_eq!(ps[0].particle_type, ParticleType::Particle);
        assert_eq!(ps[1].particle_type, ParticleType::Particle);
        assert_eq!(ps.len(), 2);
    }
}