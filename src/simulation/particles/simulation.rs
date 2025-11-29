use std::isize;

use crate::{core::math::vec2::Vec2, simulation::{constraints::{boundary_constraint::{BoundaryConstraint, BoundaryConstraintVec}, contact_constraint::{ContactConstraint, ContactConstraintVec}, distance_constraint::{DistanceConstraint, DistanceConstraintVec}, gas_constraint::{GasConstraint, GasConstraintVec}, rigid_contact_constraint::{RigidContactConstraint, RigidContactConstraintVec}, spring_constraint::{SpringConstraint, SpringConstraintVec}, total_fluid_constraint::{TotalFluidConstraint, TotalFluidConstraintVec}, total_shape_constraint::TotalShapeConstraint, volume_constraint::{VolumeConstraint, VolumeConstraintVec}}, particles::{body::Body, fluid_emitter::FluidEmitter, open_smoke_emitter::OpenSmokeEmitter, particle::{Particle, Phase}, particle_vec::ParticleVec, sdf_data::SdfData, spatial_hash::SpatialHash}}};



pub struct Simulation {
    pub particles: ParticleVec,
    pub gravity: Vec2,

    pub x_boundaries: Vec2,
    pub y_boundaries: Vec2,

    pub bodies: Vec<Body>,

    pub contact_boundary_constraints: BoundaryConstraintVec,
    pub contact_rigid_contact_constraints: RigidContactConstraintVec,
    pub contact_contact_constraints: ContactConstraintVec,

    pub distance_constraints: DistanceConstraintVec,
    pub spring_constraints: SpringConstraintVec,
    pub global_standard_total_fluid_constraints: TotalFluidConstraintVec,
    pub global_standard_gas_constraints: GasConstraintVec,
    pub volume_constraints: VolumeConstraintVec,
    
    pub smoke_emitters: Vec<OpenSmokeEmitter>,
    pub fluid_emitters: Vec<FluidEmitter>,

    pub counts: Vec<usize>,
    pub body_count: usize,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            particles: ParticleVec::new(),
            gravity: Vec2::new(0.0, -9.8),

            x_boundaries: Vec2::new(-100.0,100.0),
            y_boundaries: Vec2::new(-100.0,100.0),

            bodies: vec![],

            // CONTACT group:
            contact_boundary_constraints: BoundaryConstraintVec::new(),
            contact_rigid_contact_constraints: RigidContactConstraintVec::new(),
            contact_contact_constraints: ContactConstraintVec::new(),
            // CONTACT group end.

            distance_constraints: DistanceConstraintVec::new(),
            spring_constraints: SpringConstraintVec::new(),
            global_standard_total_fluid_constraints: TotalFluidConstraintVec::new(),
            global_standard_gas_constraints: GasConstraintVec::new(),
            volume_constraints: VolumeConstraintVec::new(),

            smoke_emitters: vec![],
            fluid_emitters: vec![],

            counts: vec![],
            body_count: 0,
        }
    }

    pub fn tick_1(&mut self, time_delta: f32) {
        // https://github.com/ebirenbaum/ParticleSolver/blob/master/cpu/src/simulation.cpp

        debug_assert!(self.contact_boundary_constraints.len() == 0);
        debug_assert!(self.contact_rigid_contact_constraints.len() == 0);
        debug_assert!(self.contact_contact_constraints.len() == 0);
        debug_assert!(self.counts.len() == 0);

        // Add all rigid body shape constraints
        // FM: doesn't need to occur as we do this dynamically as required - see how I use TotalShapeConstraint below.

        // Add all other global constraints
        // for (int i = 0; i < m_globalConstraints.size(); i++) {
        //     QList<Constraint *> group = m_globalConstraints[(ConstraintGroup) i];
        //     for (int j = 0; j < group.size(); j++) {
        //         constraints[(ConstraintGroup) i].append(group.at(j));
        //     }
        // }

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
            self.counts.push(0);//m_counts[i] = 0;
            
            // (4) Apply mass scaling (used by certain constraints)
            p.scale_mass();
        }
        // (5) End for

        // m_contactSolver.setupM(&m_particles, true);


        // Use SpatialHash to speed up particle collision checking
        // Ideally we keep spatial hash as part of self to save memory reallocation.
        let mut spatial_hash = SpatialHash::<usize, 1>::new();
        for i in 0..particle_count {
            let p = &mut self.particles[i];
            let aabb = p.get_aabb();
            spatial_hash.insert_aabb(aabb, i);
        }

        // (6) For all particles
        for i in 0..particle_count {
            let p = &self.particles[i];

            // (7) Find neighboring particles and solid contacts, naive solution
            for j in spatial_hash.aabb_iter(p.get_aabb()) { //for j in (i + 1)..particle_count {
                if j <= i {
                    continue;
                }

                let p2 = &self.particles[j];

                // Skip collision between two immovables
                if p.imass == 0.0 && p2.imass == 0.0 {
                    continue;
                }
                // Skip collisions betwee particles in the same rigid body
                else if p.phase == Phase::Solid && p2.phase == Phase::Solid && p.body == p2.body && p.body != -1 {
                    continue;
                } else {
                        
                    // Collision happens when circles overlap
                    let dist = (p.pos_guess - p2.pos_guess).magnitude(); // todo: use mag2?
                    let particle_diam = p.radius + p2.radius;
                    if dist < particle_diam - f32::EPSILON {

                        // Rigid contact constraints (which include friction) apply to solid-solid contact
                        if p.phase == Phase::Solid && p2.phase == Phase::Solid {
                            self.contact_rigid_contact_constraints.push(RigidContactConstraint::new(i, j, false)); // constraints[CONTACT].append(new RigidContactConstraint(i, j, &m_bodies));
    // #ifdef USE_STABILIZATION
    //                         constraints[STABILIZATION].append(new RigidContactConstraint(i, j, &m_bodies, true));
    // #endif
                        // Regular contact constraints (which have no friction) apply to other solid-other contact
                        } else if p.phase == Phase::Solid || p2.phase == Phase::Solid {
                            self.contact_contact_constraints.push(ContactConstraint::new(i, j, false));
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
        {
            let c = TotalShapeConstraint::new();
            for i in 0..self.bodies.len() {
                let body = &self.bodies[i];
                c.update_counts(&mut self.counts, body);
            }
        }
        self.distance_constraints.update_counts(&mut self.counts);
        self.spring_constraints.update_counts(&mut self.counts);
        self.global_standard_total_fluid_constraints.update_counts(&mut self.counts);
        self.global_standard_gas_constraints.update_counts(&mut self.counts);
        self.volume_constraints.update_counts(&mut self.counts);
        self.contact_rigid_contact_constraints.update_counts(&mut self.counts);
        self.contact_contact_constraints.update_counts(&mut self.counts);
        self.contact_boundary_constraints.update_counts(&mut self.counts);

        // update_counts_callback(self);
    }

    pub fn tick_2(&mut self, time_delta: f32, _solver_iterations: i32, iteration: i32) {
        
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
        let _i = iteration;
        //let solver_iterations = 3; // user tweakable
        //for i in 0..solver_iterations {
 
            // (17) For constraint group
            //  (18, 19, 20) Solve constraints in g and update ep
            {
                let c = TotalShapeConstraint::new();
                for i in 0..self.bodies.len() {
                    let body = &mut self.bodies[i];
                    c.project(&mut self.particles, &self.counts, body);
                }
            }
            self.distance_constraints.solve(&mut self.particles, &self.counts);
            self.spring_constraints.solve(&mut self.particles, &self.counts, time_delta);
            self.global_standard_total_fluid_constraints.solve(&mut self.particles, &self.counts);
            self.global_standard_gas_constraints.solve(&mut self.particles, &self.counts);
            self.volume_constraints.solve(&mut self.particles, &self.counts, time_delta);
            self.contact_rigid_contact_constraints.solve(&mut self.particles, &self.counts, &self.bodies);
            self.contact_contact_constraints.solve(&mut self.particles, &self.counts);
            self.contact_boundary_constraints.solve(&mut self.particles, &self.counts);
            //solve_constraints_callback(self, time_delta);

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
        //}
    }

    pub fn tick_3(&mut self, time_delta: f32) {
        // (23) For all particles
        for i in 0..self.particles.len() {
            let p = &mut self.particles[i];

            // (24) Update velocities
            p.vel = (p.pos_guess - p.pos) / time_delta;

            // (25, 26) Advect diffuse particles, apply internal forces
            // TODO

            // (27) Update positions or apply sleeping
            p.confirm_guess();
        }
        // (28) End for


        // Delete temporary conact constraints
        self.contact_boundary_constraints.clear();
        self.contact_rigid_contact_constraints.clear();
        self.contact_contact_constraints.clear();
        self.counts.clear();


        for e in self.smoke_emitters.iter_mut() {
            e.tick(&mut self.particles, time_delta, &mut self.global_standard_gas_constraints);
            // (8) Find solid boundary contacts
            for i in 0..e.particles.len() {
                let p = &mut e.particles[i];

                if p.pos.x < self.x_boundaries.x {
                    p.pos.x = self.x_boundaries.x;
                } else if p.pos.x > self.x_boundaries.y {
                    p.pos.x = self.x_boundaries.y;
                }
                if p.pos.y < self.y_boundaries.x {
                    p.pos.y = self.y_boundaries.x;
                } else if p.pos.y > self.y_boundaries.y {
                    p.pos.y = self.y_boundaries.y;
                }
            }
        }

        for e in self.fluid_emitters.iter_mut() {
            e.tick(&mut self.particles, time_delta, &mut self.global_standard_total_fluid_constraints);
        }
    }

    pub fn create_rigid_body(&mut self, particles: &mut ParticleVec, sdf_data: &Vec<SdfData>) -> usize {
        if particles.len() <= 1 {
            assert!(false, "Rigid bodies must be at least 2 points.") 
        }

        let mut body = Body::new();

        let offset = self.particles.len();
        let mut total_mass = 0.0;
        for i in 0..particles.len() {
            let p = &mut particles[i];
            p.body = self.bodies.len() as isize; //self.body_count as isize; //self.bodies.len();
            self.body_count += 1;

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

        let idx = self.bodies.len();
        self.bodies.push(body);
        idx
        // return body;
    }

    pub fn create_fluid(&mut self, particles: &ParticleVec, density: f32) -> usize {
        let offset = self.particles.len();

        // todo: BUGGY!? probably want to just assign body id's 
        //let mut rng = rand::rng();
        //let r1: f32 = rng.random();
        let bod = isize::MAX - self.body_count as isize; //(r1 * 100.0) as isize; //self.body_count; //self.global_standard_total_fluid_constraints.len(); //100 * rand::rng().random(); // assign a rnadom body number to this fluid? probably just want to avoid self collisions
        self.body_count += 1;

        let mut indices = vec![];
        for i in 0..particles.len() { //for (int i = 0; i < verts->size(); i++) {
            let mut p = particles[i];
            p.set_phase(Phase::Fluid);
            p.body = bod as isize;
            // p->ph = FLUID;
            // p->bod = bod;

            if p.imass == 0.0 {
                assert!(false, "A fluid cannot have a point of infinite mass.");
            }

            self.particles.push(p);
            indices.push(offset + i);
        }

        let idx = self.global_standard_total_fluid_constraints.len();
        self.global_standard_total_fluid_constraints.push(TotalFluidConstraint::new(density, &indices));
        return idx;
    }

    pub fn create_smoke_emitter(&mut self, posn: Vec2, particles_per_sec: f32, gas_index: usize /*GasConstraint *gs*/) {
        self.smoke_emitters.push(OpenSmokeEmitter::new(posn, particles_per_sec, gas_index /*gs*/));
    }

    pub fn create_fluid_emitter(&mut self, posn: Vec2, particles_per_sec: f32, fluid_index: usize /*TotalFluidConstraint *fs*/) {
        self.fluid_emitters.push(FluidEmitter::new(posn, particles_per_sec, fluid_index));
    }

    // open = false by default
    pub fn create_gas(&mut self, particles: &ParticleVec, density: f32, open: bool) -> usize {
        let offset = self.particles.len();
        let bod = self.body_count; //100 * frand();
        self.body_count += 1;

        let mut indices = vec![];
        for i in 0..particles.len() { //for (int i = 0; i < verts->size(); i++) {
            let mut p = particles[i];
            p.set_phase(Phase::Gas);
            p.body = bod as isize;

            if p.imass == 0.0 {
                assert!(false, "A gas cannot have a point of infinite mass.");
            }

            self.particles.push(p);
            indices.push(offset + i);
        }
        let idx = self.global_standard_gas_constraints.len();
        self.global_standard_gas_constraints.push(GasConstraint::new(density, &indices, open));
        //GasConstraint *gs = new GasConstraint(density, &indices, open);
        //m_globalConstraints[STANDARD].append(gs);
        return idx;
    }

    pub fn add_distance_constraint(&mut self, c: DistanceConstraint) {
        self.distance_constraints.push(c);
    }

    pub fn add_spring_constraint(&mut self, c: SpringConstraint) {
        self.spring_constraints.push(c);
    }

    pub fn add_volume_constraint(&mut self, c: VolumeConstraint) {
        self.volume_constraints.push(c);
    }

    pub fn add_particle(&mut self, p: Particle) {
        self.particles.push(p);
    }
}