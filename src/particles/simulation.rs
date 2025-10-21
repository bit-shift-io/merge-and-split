use cgmath::InnerSpace;

use crate::{math::vec2::Vec2, particles::{particle::Phase, particle_vec::ParticleVec}};



pub struct Simulation {
    pub particles: ParticleVec,
    pub gravity: Vec2,

    pub x_boundaries: Vec2,
    pub y_boundaries: Vec2,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            particles: ParticleVec::new(),
            gravity: Vec2::new(0.0, -9.8),

            x_boundaries: Vec2::new(-20.0,20.0),
            y_boundaries: Vec2::new(0.0,1000000.0),
        }
    }

    pub fn tick(self: &mut Self, time_delta: f32) {
        // https://github.com/ebirenbaum/ParticleSolver/blob/master/cpu/src/simulation.cpp

        // Add all rigid body shape constraints

        // Add all other global constraints

        let particle_count = self.particles.len();

        // (1) For all particles
        for i in 0..particle_count {
            let p = &mut self.particles[i];

            // (2) Apply forces
            let mut my_gravity = self.gravity;
            if p.phase == Phase::Gas {
                my_gravity *= -0.2; // Gravity scaling factor for gases - todo: make user tweakable
            }

            p.vel = p.vel + time_delta * my_gravity + time_delta * p.force;
            p.force = Vec2::new(0.0, 0.0);

            // (3) Predict positions, reset n
            p.pos_guess = p.guess(time_delta);
            //m_counts[i] = 0;

            // (4) Apply mass scaling (used by certain constraints)
            p.scale_mass();
        }
        // (5) End for

        // m_contactSolver.setupM(&m_particles, true);

        // (6) For all particles
        for i in 0..particle_count {
            let p = &self.particles[i];

            // (7) Find neighboring particles and solid contacts, naive solution
            for j in (i + 1)..particle_count {
                let p2 = &self.particles[j];

                // Skip collision between two immovables
                if p.imass == 0.0 && p2.imass == 0.0 {
                    continue;
                }
                // Skip collisions betwee particles in the same rigid body
                else if p.phase == Phase::Solid && p2.phase == Phase::Solid && p.body == p2.body && p.body != usize::MAX {
                    continue;
                } else {
                        
                    // Collision happens when circles overlap
                    let dist = (p.pos_guess - p2.pos_guess).magnitude(); // todo: use mag2?
                    let particle_diam = p.radius + p2.radius;
                    if dist < particle_diam - f32::EPSILON {

                        // Rigid contact constraints (which include friction) apply to solid-solid contact
                        if p.phase == Phase::Solid && p2.phase == Phase::Solid {
    //                         constraints[CONTACT].append(new RigidContactConstraint(i, j, &m_bodies));
    // #ifdef USE_STABILIZATION
    //                         constraints[STABILIZATION].append(new RigidContactConstraint(i, j, &m_bodies, true));
    // #endif
                        // Regular contact constraints (which have no friction) apply to other solid-other contact
                        } else if p.phase == Phase::Solid || p2.phase == Phase::Solid {
                            // constraints[CONTACT].append(new ContactConstraint(i, j));
                        }
                    }
                }

                        
                // (8) Find solid boundary contacts
                if p.pos_guess.x < self.x_boundaries.x + p.radius {
        //             constraints[CONTACT].append(new BoundaryConstraint(i, m_xBoundaries.x, true, true));
        // #ifdef USE_STABILIZATION
        //             constraints[STABILIZATION].append(new BoundaryConstraint(i, m_xBoundaries.x, true, true, true));
        // #endif
                } else if p.pos_guess.x > self.x_boundaries.y - p.radius {
        //             constraints[CONTACT].append(new BoundaryConstraint(i, m_xBoundaries.y, true, false));
        // #ifdef USE_STABILIZATION
        //             constraints[STABILIZATION].append(new BoundaryConstraint(i, m_xBoundaries.y, true, false, true));
        // #endif
                }

                if p.pos_guess.y < self.y_boundaries.x + p.radius {
        //             constraints[CONTACT].append(new BoundaryConstraint(i, m_yBoundaries.x, false, true));
        // #ifdef USE_STABILIZATION
        //             constraints[STABILIZATION].append(new BoundaryConstraint(i, m_yBoundaries.x, false, true, true));
        // #endif
                } else if p.pos_guess.y > self.y_boundaries.y - p.radius {
        //             constraints[CONTACT].append(new BoundaryConstraint(i, m_yBoundaries.y, false, false));
        // #ifdef USE_STABILIZATION
        //             constraints[STABILIZATION].append(new BoundaryConstraint(i, m_yBoundaries.y, false, false, true));
        // #endif
                }
            }

        }
        // (9) End for

        // m_contactSolver.setupSizes(m_particles.size(), &constraints[STABILIZATION]);




        // (23) For all particles
        for i in 0..self.particles.len() {
            let p = &mut self.particles[i];

            // (24) Update velocities
            p.vel = (p.pos_guess - p.pos) / time_delta;

            // (25, 26) Advect diffuse particles, apply internal forces
            /// TODO

            // (27) Update positions or apply sleeping
            p.confirm_guess();
        }
        // (28) End for
    }


}