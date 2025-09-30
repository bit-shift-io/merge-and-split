use std::usize;

use cgmath::InnerSpace;

use crate::{math::vec2::Vec2, particles::{operations::operation::Operation, particle::{Particle, ParticleType}, particle_vec::ParticleVec}};

// MetaParticle enum for tree structure
enum MetaParticle {
    Leaf {
        index: usize,
        //particle: Particle,
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
    fn get_mass(&self, ps: &ParticleVec) -> f32 {
        match self {
            MetaParticle::Leaf { index } => ps[*index].mass, // Assume global PARTICLES vec
            MetaParticle::Node { mass, .. } => *mass,
        }
    }

    // Get position (for leaf, from particle; for node, stored)
    fn get_pos(&self, ps: &ParticleVec) -> Vec2 {
        match self {
            MetaParticle::Leaf { index } => ps[*index].pos,
            MetaParticle::Node { position, .. } => *position,
        }
    }

    // Get velocity
    fn get_vel(&self, ps: &ParticleVec) -> Vec2 {
        match self {
            MetaParticle::Leaf { index } => ps[*index].vel,
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
fn merge(left: MetaParticle, right: MetaParticle, ps: &ParticleVec) -> MetaParticle {
    let m1 = left.get_mass(ps);
    let m2 = right.get_mass(ps);
    let m12 = m1 + m2;

    let x1 = left.get_pos(ps);
    let x2 = right.get_pos(ps);
    let x12 = ((m1 / m12) * x1) + ((m2 / m12) * x2);

    let v1 = left.get_vel(ps);
    let v2 = right.get_vel(ps);
    let v12 = ((m1 / m12) * v1) + ((m2 / m12) * v2);

    let delta_v = v1 - v2;
    let delta_e = (m1 * m2 / (2.0 * m12)) * delta_v.magnitude2();

    let n = x2 - x1;

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
        MetaParticle::Leaf { index: indices[0] }
    } else if indices.len() == 2 {
        let left = MetaParticle::Leaf { index: indices[0] };
        let right = MetaParticle::Leaf { index: indices[1] };
        merge(left, right, ps)
    } else {
        // Split into two halves for balanced tree
        let mid = indices.len() / 2;
        let left_tree = build_meta_tree(&indices[0..mid], ps);
        let right_tree = build_meta_tree(&indices[mid..], ps);
        merge(left_tree, right_tree, ps)
    }
}

fn compute_collisions(ps: &ParticleVec) -> Vec<Vec<usize>> {
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

fn execute_merge(ps: &mut ParticleVec) -> Vec<MetaParticle> {
    let mut meta_particles = vec![];
    let collisions = compute_collisions(ps);
    for ci in 0..collisions.len() {
        let particle_collisions = &collisions[ci];

        let meta_particle = build_meta_tree(particle_collisions, ps);
        meta_particles.push(meta_particle);
    }

    meta_particles
}

// Function to split a meta and update particle positions/velocities
// alpha: restitution coefficient
fn split(meta: &mut MetaParticle, alpha: f32, ps: &mut ParticleVec) {
    match meta {
        MetaParticle::Leaf { index } => {
            // Nothing to do for leaf
        }
        MetaParticle::Node {
            left,
            right,
            mass: m12,
            position: x12,
            velocity: v12,
            delta_e,
            n,
            v_left_initial: v1,
            v_right_initial: v2,
            ..
        } => {
            let m1 = left.get_mass(ps);
            let m2 = right.get_mass(ps);

            // Compute positions for children
            let hat_n = n.normalize();
            let x1_new = *x12 - ((m2 / *m12) * *n);
            let x2_new = *x12 + ((m1 / *m12) * *n);

            // Set positions recursively
            set_positions_recursive(left, x1_new, ps);
            set_positions_recursive(right, x2_new, ps);

            // Now compute velocities
            let s_sq = 2.0 * alpha * *delta_e / *m12 * (m1 / m2);
            let s = s_sq.sqrt();

            // Try epsilon = 0 first
            let dv = *v12 - *v1;
            let mu_proj = hat_n.dot(dv);
            let a = 1.0;
            let b = -2.0 * mu_proj;
            let c = dv.magnitude2() - s_sq;
            let discriminant = b * b - 4.0 * a * c;

            let mut mu = 0.0;
            let mut epsilon = Vec2::new(0.0, 0.0);

            if discriminant >= 0.0 {
                let sqrt_d = discriminant.sqrt();
                let mu1 = ( -b - sqrt_d ) / (2.0 * a);
                let mu2 = ( -b + sqrt_d ) / (2.0 * a);
                // Choose smaller mu
                mu = mu1.min(mu2);
                // Check separation condition
                // let check = ; //sub(add(*v1, scale(mu, hat_n)), *v12) * (-m12 / m2)).dot(hat_n);
                // if check < 0.0 {
                //     mu = mu2; // If not, take larger? But paper says take smaller for separation
                // }
            } else {
                // No real root, use geometric solution
                mu = hat_n.dot(*v12 - *v1);
                let w = (*v12 - *v1) - (mu * hat_n);
                let w_len = w.magnitude();
                if w_len > 0.0 {
                    epsilon = (w_len - s) * w.normalize();
                }
            }

            let v1_new = *v1 + (mu * hat_n) + epsilon;

            let v2_new = ((*m12 / m2) * *v12) + ((-m1 / m2) * v1_new);

            // Set velocities recursively
            set_velocities_recursive(left, v1_new, ps);
            set_velocities_recursive(right, v2_new, ps);

            // Recurse
            split(left, alpha, ps);
            split(right, alpha, ps);
        }
    }
}

// Helper to set position recursively
fn set_positions_recursive(meta: &mut MetaParticle, pos: Vec2, ps: &mut ParticleVec) {
    match meta {
        MetaParticle::Leaf { index } => {
            ps[*index].pos = pos;
        },
        MetaParticle::Node { .. } => meta.set_pos(pos),
    }
}

// Helper to set velocity recursively
fn set_velocities_recursive(meta: &mut MetaParticle, vel: Vec2, ps: &mut ParticleVec) {
    match meta {
        MetaParticle::Leaf { index } => {
            ps[*index].vel = vel;
        },
        MetaParticle::Node { .. } => meta.set_vel(vel),
    }
}

#[cfg(test)]
mod tests {
    use crate::{math::vec2::Vec2, particles::operations::metrics::Metrics};

    use super::*;

    #[test]
    fn merge_and_split_2_intersecting() {
        let p1 = *Particle::default().set_pos(Vec2::new(0.0, 0.0)).set_vel(Vec2::new(0.1, 0.0));
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0));

        let mut ps = ParticleVec::from([p1, p2]);

        let mut met1 = Metrics::default();
        met1.execute(&mut ps);

        // merge particles into MetaParticles
        let meta_particles = execute_merge(&mut ps);

        // split MetaParticles
        for mut meta in meta_particles {
            let alpha = 1.0;
            split(&mut meta, alpha, &mut ps);
        }

        let mut met2 = Metrics::default();
        met2.execute(&mut ps);

        // This assert is passing. Both kinetic energy and momentum are conserved when dealing with only 2 particles being combined into a single MetaParticle.
        assert!(met1.approx_equal(&met2));
    }


    #[test]
    fn merge_and_split_3_intersecting() {
        let p1 = *Particle::default().set_pos(Vec2::new(0.0, 0.0)).set_vel(Vec2::new(0.1, 0.0)); // At origin.
        let p2 = *Particle::default().set_pos(Vec2::new(0.9, 0.0)); // To the right of p1 such that it just overlaps.
        let p3 = *Particle::default().set_pos(Vec2::new(0.5, 0.5)); // Between p1 and p2, but higher, so all 3 overlap.

        let mut ps = ParticleVec::from([p1, p2, p3]);

        let mut met1 = Metrics::default();
        met1.execute(&mut ps);

        // merge particles into meta particles
        let meta_particles = execute_merge(&mut ps);

        // split meta particles
        for mut meta in meta_particles {
            let alpha = 1.0;
            split(&mut meta, alpha, &mut ps);
        }

        let mut met2 = Metrics::default();
        met2.execute(&mut ps);

        // This assert is failing. Momentum is conserved, but kinetic energy is lost. Therefore, there is a problem when dealing with recursion. 
        assert!(met1.approx_equal(&met2));
    }

}