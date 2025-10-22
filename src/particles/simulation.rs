use cgmath::InnerSpace;

use crate::{constraints2::boundary_constraint::BoundaryConstraint, math::vec2::Vec2, particles::{sdf_data::SdfData, body::Body, particle::{Particle, Phase}, particle_vec::ParticleVec}};



pub struct Simulation {
    pub particles: ParticleVec,
    pub gravity: Vec2,

    pub x_boundaries: Vec2,
    pub y_boundaries: Vec2,

    pub bodies: Vec<Body>,

    pub contact_boundary_constraints: Vec<BoundaryConstraint>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            particles: ParticleVec::new(),
            gravity: Vec2::new(0.0, -9.8),

            x_boundaries: Vec2::new(-20.0,20.0),
            y_boundaries: Vec2::new(0.0,1000000.0),

            bodies: vec![],

            // CONTACT group:
            contact_boundary_constraints: vec![],
        }
    }

    pub fn tick(self: &mut Self, time_delta: f32) {
        // https://github.com/ebirenbaum/ParticleSolver/blob/master/cpu/src/simulation.cpp

        // Add all rigid body shape constraints

        // Add all other global constraints

        let mut counts = Vec::<usize>::new();
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
            counts.push(0);//m_counts[i] = 0;
            
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
            }
    
            // (8) Find solid boundary contacts
            if p.pos_guess.x < self.x_boundaries.x + p.radius {
                self.contact_boundary_constraints.push(BoundaryConstraint::new(i, self.x_boundaries.x, true, true, false));
    // #ifdef USE_STABILIZATION
    //             constraints[STABILIZATION].append(new BoundaryConstraint(i, m_xBoundaries.x, true, true, true));
    // #endif
            } else if p.pos_guess.x > self.x_boundaries.y - p.radius {
                self.contact_boundary_constraints.push(BoundaryConstraint::new(i, self.x_boundaries.y, true, false, false));
    // #ifdef USE_STABILIZATION
    //             constraints[STABILIZATION].append(new BoundaryConstraint(i, m_xBoundaries.y, true, false, true));
    // #endif
            }

            if p.pos_guess.y < self.y_boundaries.x + p.radius {
                self.contact_boundary_constraints.push(BoundaryConstraint::new(i, self.y_boundaries.x, false, true, false));
    // #ifdef USE_STABILIZATION
    //             constraints[STABILIZATION].append(new BoundaryConstraint(i, m_yBoundaries.x, false, true, true));
    // #endif
            } else if p.pos_guess.y > self.y_boundaries.y - p.radius {
                self.contact_boundary_constraints.push(BoundaryConstraint::new(i, self.y_boundaries.y, false, false, false));
    // #ifdef USE_STABILIZATION
    //             constraints[STABILIZATION].append(new BoundaryConstraint(i, m_yBoundaries.y, false, false, true));
    // #endif
            }
        }
        // (9) End for

        // m_contactSolver.setupSizes(m_particles.size(), &constraints[STABILIZATION]);



        // (17) For constraint group
        for c in self.contact_boundary_constraints.iter_mut() {
            c.update_counts(&mut counts)
        }
        // for (int j = 0; j < (int) NUM_CONSTRAINT_GROUPS; j++) {
        //     ConstraintGroup g = (ConstraintGroup) j;

        //     // Skip the stabilization constraints
        //     if (g == STABILIZATION) {
        //         continue;
        //     }

        //     //  (18, 19, 20) Update n based on constraints in g
        //     for (int k = 0; k < constraints[g].size(); k++) {
        //         constraints[g].at(k)->updateCounts(m_counts);
        //     }
        // }



        // // (16) For solver iterations
        let solver_iterations = 3; // user tweakable
        for i in 0..solver_iterations {
 
            // (17) For constraint group
            //  (18, 19, 20) Solve constraints in g and update ep
            for c in self.contact_boundary_constraints.iter_mut() {
                c.project(&mut self.particles, &counts)
            }

        //     for (int j = 0; j < (int) NUM_CONSTRAINT_GROUPS; j++) {
        //         ConstraintGroup g = (ConstraintGroup) j;

        //         // Skip the stabilization constraints
        //         if (g == STABILIZATION) {
        //             continue;
        //         }

        //         //  (18, 19, 20) Solve constraints in g and update ep
        //         for (int k = 0; k < constraints[g].size(); k++) {
        //             constraints[g].at(k)->project(&m_particles, m_counts);
        //         }
        //     }
        }

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


        // Delete temporary conact constraints
        self.contact_boundary_constraints.clear();
    }

    pub fn create_rigid_body(&mut self, particles: &mut ParticleVec, sdf_data: &Vec<SdfData>) {
        if particles.len() <= 1 {
            assert!(false, "Rigid bodies must be at least 2 points.") 
        }

        let mut body = Body::new();

        let offset = self.particles.len();
        let mut total_mass = 0.0;
        for i in 0..particles.len() {
            let p = &mut particles[i];
            p.body = self.bodies.len();
            p.phase = Phase::Solid;

            if p.imass == 0.0 {
               assert!(false, "A rigid body cannot have a point of infinite mass.") 
            }

            total_mass += 1.0 / p.imass;

            self.particles.push(*p);
            body.particle_indicies.push(i + offset);
            body.sdf.insert(i + offset, sdf_data[i]);
        }

        // Update the body's global properties, including initial r_i vectors
        body.imass = 1.0 / total_mass;
        body.update_com(&self.particles, false);
        body.compute_rs(&self.particles);
        // body->shape = new TotalShapeConstraint(body);

        self.bodies.push(body);
        // return body;
    }


    pub fn init_friction(&mut self) {
        self.x_boundaries = Vec2::new(-20.0,20.0);
        self.y_boundaries = Vec2::new(0.0,1000000.0);


        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let root2 = f32::sqrt(2.0);
        let mut sdf_data = Vec::<SdfData>::new();
        sdf_data.push(SdfData::new(Vec2::new(-1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(-1.0, 1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(0.0, -1.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(0.0, 1.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(1.0, 1.0).normalize(), particle_rad * root2));
        
        let x_max = 3;
        let y_max = 2;

        let mut particles = ParticleVec::new();
        for x in 0..x_max {
            let x_val = particle_diam * ((x % x_max) as f32 - x_max as f32 / 2.0);
            for y in 0..y_max {
                let y_val = (y_max + (y % y_max) + 1) as f32 * particle_diam;
                let mass = if x == 0 && y == 0 { 1.0 } else { 1.0 };
                let mut part = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x_val, y_val)).set_mass_2(mass);
                part.vel.x = 5.0;
                part.k_friction = 0.01;
                part.s_friction = 0.1;
                particles.push(part);
            }
        }

        self.create_rigid_body(&mut particles, &sdf_data);
    }
}