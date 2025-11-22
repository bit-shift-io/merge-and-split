use rand::Rng;

use crate::{core::math::{vec2::Vec2, vec4::Vec4}, simulation::{constraints::distance_constraint::DistanceConstraint, particles::{particle::{Particle, Phase}, particle_vec::ParticleVec, sdf_data::SdfData, simulation::Simulation}}};

pub struct SimulationDemos {
}

impl SimulationDemos {
    pub fn init_friction(sim: &mut Simulation) {
        sim.x_boundaries = Vec2::new(-20.0,20.0);
        sim.y_boundaries = Vec2::new(0.0,1000000.0);

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

        sim.create_rigid_body(&mut particles, &sdf_data);
    }

    pub fn init_granular(sim: &mut Simulation) {
        sim.x_boundaries = Vec2::new(-100.0,100.0);
        sim.y_boundaries = Vec2::new(-5.0,1000.0);

        let particle_rad = 0.25;
        let particle_diam = 0.5;

        for i in -10..10 {
            for j in 0..10 {
                let pos = Vec2::new((i as f32) * (particle_diam + f32::EPSILON), (j as f32).powf(1.2) * (particle_diam) + particle_rad + sim.y_boundaries.x);
                let mut part= *Particle::default().set_radius(particle_rad).set_pos(pos).set_mass_2(1.0); //, 1, SOLID);
                part.phase = Phase::Solid;
                part.s_friction = 0.35;
                part.k_friction = 0.3;
                sim.add_particle(part);
            }
        }

        let mut jerk = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(-25.55, 40.0)).set_mass_2(100.0);
        jerk.phase = Phase::Solid;
        jerk.vel.x = 8.5;
        jerk.set_colour(Vec4::RED);
        sim.add_particle(jerk);
    }


    pub fn init_sdf(sim: &mut Simulation) {
        sim.x_boundaries = Vec2::new(-20.0,20.0);
        sim.y_boundaries = Vec2::new(0.0,1000000.0);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let num_boxes = 2;
        let root2 = f32::sqrt(2.0);

        let mut particles = ParticleVec::new();
        
        let mut sdf_data = Vec::<SdfData>::new();
        sdf_data.push(SdfData::new(Vec2::new(-1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(-1.0, 0.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(-1.0, 1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(1.0, 0.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(1.0, 1.0).normalize(), particle_rad * root2));
        
        let x_max = 3;
        let y_max = 2;
        for i in (0..num_boxes).rev() {//for (int i = numBoxes - 1; i >= 0; i--) {
            for x in 0..x_max { //for (int x = 0; x < dim.x; x++) {
                let x_val = particle_diam * ((x % x_max) as f32 - x_max as f32 / 2.0) + (i as f32) * particle_rad;
                for y in 0..y_max { //for (int y = 0; y < dim.y; y++) {
                    let y_val = ((40.0 * i as f32) * y_max as f32 + (y % y_max) as f32 + 1.0) * particle_diam;
                    let mut part = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x_val, y_val)).set_mass_2(4.0);
                    if i > 0 {
                        part.vel.y = -120.0;
                    }
                    particles.push(part);
                }
            }
            sim.create_rigid_body(&mut particles, &sdf_data);
            particles.clear();
        }
    }

    pub fn init_boxes(sim: &mut Simulation) {
        sim.x_boundaries = Vec2::new(-20.0,20.0);
        sim.y_boundaries = Vec2::new(0.0,1000000.0);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;
        
        let num_boxes = 5;
        let num_columns = 2;
        
        let root2 = f32::sqrt(2.0);

        let mut sdf_data = Vec::<SdfData>::new();
        sdf_data.push(SdfData::new(Vec2::new(-1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(-1.0, 1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(0.0, -1.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(0.0, 1.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(1.0, 1.0).normalize(), particle_rad * root2));
        
        for j in -(num_columns/2)..(num_columns/2) {
            let x_max = 3;
            let y_max = 2;

            for i in (0..(num_boxes-1)).rev() {
                let mut particles = ParticleVec::new();
                for x in 0..x_max {
                    let x_val = j as f32 * 4.0 + particle_diam * ((x % x_max) as f32 - x_max as f32 / 2.0);
                    for y in 0..y_max {
                        let y_val = ((2.0 * i as f32 + 1.0) * y_max as f32 + (y % y_max) as f32 + 1.0) as f32 * particle_diam;
                        let mass = 4.0;
                        let mut part = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x_val, y_val)).set_mass_2(mass);
                        part.k_friction = 1.0;
                        part.s_friction = 1.0;
                        particles.push(part);
                    }
                }

                sim.create_rigid_body(&mut particles, &sdf_data);

                // for (int x = 0; x < dim.x; x++) {
                //     double x_val = j * 4 + PARTICLE_DIAM * ((x % dim.x) - dim.x / 2);
                //     for (int y = 0; y < dim.y; y++) {
                //         double y_val = ((2 * i + 1) * dim.y + (y % dim.y) + 1) * PARTICLE_DIAM;
                //         Particle *part = new Particle(glm::dvec2(x_val, y_val), 4.);
                //         part->sFriction = 1.;
                //         part->kFriction = 1.;
                //         particles.push(part);
                //     }
                // }
                // Body *body = createRigidBody(&vertices, &data);
                // vertices.clear();
            }
        }
    }

    pub fn init_wall(sim: &mut Simulation) {
        sim.x_boundaries = Vec2::new(-50.0,50.0);
        sim.y_boundaries = Vec2::new(0.0,1000000.0);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let x_max = 6;
        let y_max = 2;

        let height = 10; //11;
        let width = 3; //5;

        let root2 = f32::sqrt(2.0);

        let mut particles = ParticleVec::new();
        
        let mut sdf_data = Vec::<SdfData>::new();
        sdf_data.push(SdfData::new(Vec2::new(-1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(-1.0, 1.0).normalize(), particle_rad * root2));

        for i in 0..(x_max - 2) {
            sdf_data.push(SdfData::new(Vec2::new(0.0, -1.0).normalize(), particle_rad));
            sdf_data.push(SdfData::new(Vec2::new(0.0, 1.0).normalize(), particle_rad));
        }

        sdf_data.push(SdfData::new(Vec2::new(1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(1.0, 1.0).normalize(), particle_rad * root2));
        

        for j in -width..width { //(int j = -width; j <= width; j++) {
            for i in (0..height).rev() { //(int i = height - 1; i >= 0; i--) {
                for x in 0..x_max {
                    let num = if i % 2 == 0 { 3.0 } else { -1.0 };
                    let x_val = j as f32 * (f32::EPSILON + x_max as f32 / 2.0) + particle_diam * (x % x_max) as f32 - num * particle_rad;
                    for y in 0..y_max {
                        let y_val = (i as f32 * y_max as f32 + (y % y_max) as f32 + f32::EPSILON) * particle_diam + particle_rad;
                        let mut part = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x_val, y_val)).set_mass_2(1.0);
                        part.s_friction = 1.0;
                        part.k_friction = 0.0;
                        particles.push(part);
                    }
                }
                sim.create_rigid_body(&mut particles, &sdf_data);
                particles.clear();
            }
        }
    }


    pub fn init_pendulum(sim: &mut Simulation) {
        sim.x_boundaries = Vec2::new(-10.0,10.0);
        sim.y_boundaries = Vec2::new(0.0,1000000.0);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let chain_length = 3;
        let pos = Vec2::new(0.0, chain_length as f32 * 3.0 + 6.0) * particle_diam + Vec2::new(0.0,2.0);
        let mut particle = *Particle::default().set_radius(particle_rad).set_pos(pos).set_mass_2(0.0);
        particle.phase = Phase::Solid;
        sim.add_particle(particle);
        
        let mut sdf_data = Vec::<SdfData>::new();
        sdf_data.push(SdfData::new(Vec2::new(-1.0, -1.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(-1.0, 1.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(0.0, -1.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(0.0, 1.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(1.0, -1.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(1.0, 1.0).normalize(), particle_rad));

        let mut particles = ParticleVec::new();
        let xs = [-1.0,-1.0,0.0,0.0,1.0,1.0];

        for i in (0..=chain_length).rev() { //for (int i = chain_length; i >= 0; i--) {
            for j in 0..6 { //for (int j = 0; j < 6; j++) {
                let y = ((i + 1) * 3 + (j % 2)) as f32 * particle_diam + 2.0;
                let mut part = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(xs[j] * particle_diam, y)).set_mass_2(1.0);
                part.vel.x = 3.0;
                particles.push(part);
            }
            sim.create_rigid_body(&mut particles, &sdf_data);
            particles.clear();

            if i < chain_length {
                let base_prev = 1 + (chain_length - i - 1) * 6;
                let base_cur = base_prev + 6;
                sim.add_distance_constraint(DistanceConstraint::from_particles(base_cur + 1, base_prev, &sim.particles));
                sim.add_distance_constraint(DistanceConstraint::from_particles(base_cur + 5, base_prev + 4, &sim.particles));
            }
        }

        sim.add_distance_constraint(DistanceConstraint::from_particles(0, 4, &sim.particles));
    }

    pub fn init_rope(sim: &mut Simulation) {
        let scale = 5.0;

        sim.x_boundaries = Vec2::new(-scale,scale);
        sim.y_boundaries = Vec2::new(0.0,1000000.0);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let top = 6.0;
        let dist = particle_rad;

        let mut e1 = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(sim.x_boundaries.x, top)).set_mass_2(0.0).set_phase(Phase::Solid);
        e1.body = -2; // -2?!
        sim.add_particle(e1);

        let mut i = sim.x_boundaries.x;
        while i < (sim.x_boundaries.y - dist) { //for (double i = m_xBoundaries.x + dist; i < m_xBoundaries.y - dist; i += dist) {
            let part = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(i, top)).set_mass_2(1.0).set_phase(Phase::Solid);
            //part->bod = -2;
            sim.add_particle(part);
            sim.add_distance_constraint(DistanceConstraint::new(dist, sim.particles.len() - 2, sim.particles.len() - 1, false));

            i += dist;
        }

        let mut e2 = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(sim.x_boundaries.y, top)).set_mass_2(0.0).set_phase(Phase::Solid);
        e2.body = -2;
        sim.add_particle(e2);

        sim.add_distance_constraint(DistanceConstraint::new(dist, sim.particles.len() - 2, sim.particles.len() - 1, false));
        
        let delta = 0.7;
        let mut particles = ParticleVec::new();
        let blue = Vec4::BLUE;

        let mut x = -scale;
        while x < scale { //for(double x = -scale; x < scale; x += delta) {
            let mut y= 10.0;
            while y < 10.0 + scale { //for(double y = 10; y < 10 + scale; y += delta) {
                let mut rng = rand::rng();
                let r1: f32 = rng.random();
                let r2: f32 = rng.random();

                particles.push(*Particle::default().set_colour(blue).set_radius(particle_rad).set_pos(Vec2::new(x,y) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2(1.0));
                y += delta;
            }

            x += delta;
        }
        sim.create_fluid(&particles, 1.75);
    }


    pub fn init_fluid(sim: &mut Simulation) {
        let scale = 4.0;
        let delta = 0.7;

        sim.x_boundaries = Vec2::new(-2.0 * scale,2.0 * scale);
        sim.y_boundaries = Vec2::new(-2.0 * scale, 3.0 * scale);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let mut particles = ParticleVec::new();

        let mut rng = rand::rng();

        let num = 2.0;
        let mut d = 0.0;
        while d < num { //(int d = 0; d < num; d++) {
            let color = Vec4::new(rng.random(), rng.random(), rng.random(), 1.0);
            let start = -2.0 * scale + 4.0 * scale * (d / num);
            let mut x = start;
            while x < start + (4.0 * scale / num) { // for(double x = start; x < start + (4 * scale / num); x += delta) {
                let mut y = -2.0 * scale;
                while y < scale { //for(double y = -2 * scale; y < scale; y += delta) {
                    let r1: f32 = rng.random();
                    let r2: f32 = rng.random();

                    particles.push(*Particle::default().set_radius(particle_rad).set_colour(color).set_pos(Vec2::new(x, y) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2(1.0));

                    y += delta;
                }

                x += delta;
            }
            sim.create_fluid(&particles, 1.0 + 0.75 * d);
            particles.clear();

            d += 1.0;
        }
    }


    pub fn init_fluid_solid(sim: &mut Simulation) {
        let scale = 3.0;
        let delta = 0.7;
        
        sim.x_boundaries = Vec2::new(-2.0 * scale,2.0 * scale);
        sim.y_boundaries = Vec2::new(-2.0 * scale, 100.0 * scale);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let mut particles = ParticleVec::new();

        let blue = Vec4::BLUE;

        let num = 1.0;
        let mut d = 0.0;
        while d < num { //for (int d = 0; d < num; d++) {
            let start = -2.0 * scale + 4.0 * scale * (d / num);
            let mut x = start;
            while x < start + (4.0 * scale / num) { //for(double x = start; x < start + (4 * scale / num); x += delta) {
                let mut y = -2.0 * scale;
                while y < 2.0 * scale { //for(double y = -2 * scale; y < 2 * scale; y += delta) {
                    let mut rng = rand::rng();
                    let r1: f32 = rng.random();
                    let r2: f32 = rng.random();

                    particles.push(*Particle::default().set_colour(blue).set_radius(particle_rad).set_pos(Vec2::new(x,y + 3.0) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2(1.0));

                    y += delta;
                }

                x += delta;
            }
            sim.create_fluid(&particles, 1.0 + 1.25 * (d + 1.0));
            particles.clear();

            d += 1.0;
        }


        let root2 = f32::sqrt(2.0);

        if true {
            particles.clear();

            let dim_x = 5;
            let dim_y = 2;

            let mut sdf_data = Vec::<SdfData>::new();
            sdf_data.push(SdfData::new(Vec2::new(-1.0, -1.0).normalize(), particle_rad * root2));
            sdf_data.push(SdfData::new(Vec2::new(-1.0, 1.0).normalize(), particle_rad * root2));
    
            for i in 0..(dim_x - 2) { //for (int i = 0; i < dim.x - 2; i++) {
                sdf_data.push(SdfData::new(Vec2::new(0.0, -1.0).normalize(), particle_rad));
                sdf_data.push(SdfData::new(Vec2::new(0.0, 1.0).normalize(), particle_rad));
            }
            sdf_data.push(SdfData::new(Vec2::new(1.0, -1.0).normalize(), particle_rad * root2));
            sdf_data.push(SdfData::new(Vec2::new(1.0, 1.0).normalize(), particle_rad * root2));
    
            for x in 0..dim_x { //(int x = 0; x < dim.x; x++) {
                let x_val = particle_diam * ((x % dim_x) as f32 - dim_x as f32 / 2.0);
                for y in 0..dim_y { //(int y = 0; y < dim.y; y++) {
                    let y_val = (dim_y as f32 + (y % dim_y) as f32 + 1.0) * particle_diam;
                    particles.push(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x_val-3.0, y_val + 10.0)).set_mass_2(2.0));
                }
            }
            sim.create_rigid_body(&mut particles, &sdf_data);
        }

        if true {
            particles.clear();

            let mut sdf_data = Vec::<SdfData>::new();
            
            let dim_x = 5;
            let dim_y = 2;

            sdf_data.push(SdfData::new(Vec2::new(-1.0, 1.0).normalize(), particle_rad * root2));
            sdf_data.push(SdfData::new(Vec2::new(-1.0, 1.0).normalize(), particle_rad * root2));
    
            for i in 0..(dim_x - 2) { //(int i = 0; i < dim.x - 2; i++) {
                sdf_data.push(SdfData::new(Vec2::new(0.0, -1.0).normalize(), particle_rad));
                sdf_data.push(SdfData::new(Vec2::new(0.0, 1.0).normalize(), particle_rad));
            }
            sdf_data.push(SdfData::new(Vec2::new(1.0, -1.0).normalize(), particle_rad * root2));
            sdf_data.push(SdfData::new(Vec2::new(1.0, 1.0).normalize(), particle_rad * root2));

            for x in 0..dim_x { //(int x = 0; x < dim.x; x++) {
                let x_val = particle_diam * ((x % dim_x) as f32 - dim_x as f32 / 2.0);
                for y in 0..dim_y {//(int y = 0; y < dim.y; y++) {
                    let y_val = (dim_y as f32 + (y % dim_y) as f32 + 1.0) * particle_diam;
                    particles.push(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x_val+3.0, y_val + 10.0)).set_mass_2(0.2));
                }
            }
            sim.create_rigid_body(&mut particles, &sdf_data);
        }
    }

    pub fn init_gas(sim: &mut Simulation) {
        let mut scale = 2.0;
        let delta = 0.7;
        
        sim.x_boundaries = Vec2::new(-2.0 * scale,2.0 * scale);
        sim.y_boundaries = Vec2::new(-2.0 * scale, 5.0 * scale);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let mut particles = ParticleVec::new();
        let mut rng = rand::rng();

        let num = 2.0;
        let mut d = 0.0;
        while d < num { //for (int d = 0; d < num; d++) {
            let color = Vec4::new(rng.random(), rng.random(), rng.random(), 1.0);
            let start = -2.0 * scale + 4.0 * scale * (d / num);
            let mut x = start;
            while x < start + (4.0 * scale / num) { //for(double x = start; x < start + (4 * scale / num); x += delta) {
                let mut y = -2.0 * scale;
                while y < 2.0 * scale { // for(double y = -2 * scale; y < 2 * scale; y += delta) {
                    
                    let r1: f32 = rng.random();
                    let r2: f32 = rng.random();

                    particles.push(*Particle::default().set_colour(color).set_radius(particle_rad).set_pos(Vec2::new(x,y) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2(1.0));
                    y += delta;
                }

                x += delta;
            }
            sim.create_gas(&particles, 0.75 + 3.0*(d), false);
            particles.clear();

            d += 1.0;
        }

        scale = 3.0;
        let mut d = 0.0;
        while d < num { //for (int d = 0; d < num; d++) {
            let color = Vec4::new(rng.random(), rng.random(), rng.random(), 1.0);
            let start = -2.0 * scale + 4.0 * scale * (d / num);
            let mut x = start;
            while x < start + (4.0 * scale / num) { //for(double x = start; x < start + (4 * scale / num); x += delta) {
                let mut y = -2.0 * scale;
                while y < 2.0 * scale { //for(double y = -2 * scale; y < 2 * scale; y += delta) {
                    let mut rng = rand::rng();
                    let r1: f32 = rng.random();
                    let r2: f32 = rng.random();

                    particles.push(*Particle::default().set_colour(color).set_radius(particle_rad).set_pos(Vec2::new(x,y+10.0) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2(1.0));
                    y += delta
                }
                x += delta;
            }
            sim.create_fluid(&particles, 4.0 + 0.75 * (d + 1.0));
            particles.clear();

            d += 1.0;
        }
    }


    pub fn init_water_balloon(sim: &mut Simulation) {
        let scale = 10.0;

        sim.x_boundaries = Vec2::new(-scale,scale);
        sim.y_boundaries = Vec2::new(-10.0, 1000000.0);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let samples = 60;
        let da = 360.0 / samples as f32;

        let blue = Vec4::BLUE;
        let red = Vec4::RED;

        for i in 0..samples { //for (int i = 0; i < samples; i++) {
            let angle = f32::to_radians(i as f32 * da); //D2R(i * da);
            let mut part = *Particle::default().set_colour(blue).set_radius(particle_rad).set_pos(Vec2::new(f32::sin(angle), f32::cos(angle)) * 3.0).set_mass_2(1.0);
            part.body = -2; // ???
            let idx = sim.particles.len();
            sim.add_particle(part);

            if i > 0 {
                sim.add_distance_constraint(DistanceConstraint::from_particles(idx, idx - 1, &sim.particles));
            }
        }
        sim.add_distance_constraint(DistanceConstraint::from_particles(0, sim.particles.len() - 1, &sim.particles));
        let idk = sim.particles.len();

        for i in 0..samples { //(int i = 0; i < samples; i++) {
            let angle = f32::to_radians(i as f32 * da);
            let mut part = *Particle::default().set_colour(red).set_radius(particle_rad).set_pos(Vec2::new(f32::sin(angle), f32::cos(angle) + 3.0) * 3.0).set_mass_2(1.0);
            part.body = -3; // ?? I think this just stops collisions without having a "body" assigned
            let idx = sim.particles.len();
            sim.add_particle(part);

            if i > 0 {
                sim.add_distance_constraint(DistanceConstraint::from_particles(idx, idx - 1, &sim.particles));
            }
        }
        sim.add_distance_constraint(DistanceConstraint::from_particles(idk, sim.particles.len() - 1, &sim.particles));

        let delta = 1.5 * particle_rad;

        let mut particles = ParticleVec::new();
        let mut rng = rand::rng();

        let mut x = -2.0;
        while x <= 2.0 {
            let mut y = -2.0;
            while y <= 2.0 {
                let r1: f32 = rng.random();
                let r2: f32 = rng.random();

                particles.push(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x,y) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2(1.0));
                y += delta;
            }
            x += delta;
        }
        sim.create_fluid(&particles, 1.75);

        particles.clear();
        let mut x = -2.0;
        while x <= 2.0 {
            let mut y = -2.0;
            while y <= 2.0 {
                let r1: f32 = rng.random();
                let r2: f32 = rng.random();

                particles.push(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x,y + 9.0) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2(1.0));
                y += delta;
            }
            x += delta;
        }
        sim.create_fluid(&particles, 1.75);
    }


    pub fn init_newtons_cradle(sim: &mut Simulation) {
        sim.x_boundaries = Vec2::new(-10.0,10.0);
        sim.y_boundaries = Vec2::new(-5.0, 1000000.0);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let n = 2;

        for i in -2..=n {
            let idx = sim.particles.len();
            sim.add_particle(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(i as f32 * particle_diam, 0.0)).set_mass_2(0.0));
            if i != -n {
                sim.add_particle(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(i as f32 * particle_diam, -3.0)).set_mass_2(1.0));
            } else {
                let part = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(i as f32 * particle_diam - 3.0, 0.0)).set_mass_2(1.0);
                sim.add_particle(part);
            }
            sim.add_distance_constraint(DistanceConstraint::from_particles(idx, idx + 1, &sim.particles));
        }
    }



    // I think open here means it is "open" such that more particles can be added.
    pub fn init_smoke_open(sim: &mut Simulation) {
        let scale = 2.0; 
        let delta = 0.63;

        sim.x_boundaries = Vec2::new(-3.0 * scale,3.0 * scale);
        sim.y_boundaries = Vec2::new(-2.0 * scale,100.0 * scale);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let mut particles = ParticleVec::new();
        let mut rng = rand::rng();

        let start = -2.0 * scale;
        let mut x = start;
        while x < start + (4.0 * scale) { //for(double x = start; x < start + (4 * scale); x += delta) {
            let mut y = -2.0 * scale;
            while y <  2.0 * scale { //for(double y = -2 * scale; y < 2 * scale; y += delta) {
                let r1: f32 = rng.random();
                let r2: f32 = rng.random();

                particles.push(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x,y) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2(1.0));

                y += delta;
            }

            x += delta;
        }
        //GasConstraint *gs = 
        let gas_idx = sim.create_gas(&particles, 1.5, true);
        particles.clear();

        sim.create_smoke_emitter(Vec2::new(0.0,-2.0 * scale + 1.0), 15.0, gas_idx /*gs*/);
    }


    // I think closed here means it is "closed" such that no more particles can be added.
    pub fn init_smoke_closed(sim: &mut Simulation) {
        let scale = 2.0; 
        let delta = 0.63;

        sim.x_boundaries = Vec2::new(-3.0 * scale,3.0 * scale);
        sim.y_boundaries = Vec2::new(-2.0 * scale,100.0 * scale);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let mut particles = ParticleVec::new();
        let mut rng = rand::rng();

        let start = -2.0 * scale;
        let mut x = start;
        while x < start + (4.0 * scale) { //for(double x = start; x < start + (4 * scale); x += delta) {
            let mut y = -2.0 * scale;
            while y <  2.0 * scale { //for(double y = -2 * scale; y < 2 * scale; y += delta) {
                let r1: f32 = rng.random();
                let r2: f32 = rng.random();

                particles.push(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x,y) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2(1.0));

                y += delta;
            }

            x += delta;
        }
        //GasConstraint *gs = 
        let gas_idx = sim.create_gas(&particles, 1.5, false);
        particles.clear();

        sim.create_smoke_emitter(Vec2::new(0.0,-2.0 * scale + 1.0), 15.0, usize::MAX /*gs*/);
    }


    pub fn init_rope_gas(sim: &mut Simulation) {
        let scale = 2.0;
        let delta = 0.63;

        sim.x_boundaries = Vec2::new(-4.0 * scale,4.0 * scale);
        sim.y_boundaries = Vec2::new(-2.0 * scale,100.0 * scale);

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let red = Vec4::RED;

        //let mut particles = ParticleVec::new();
        let mut rng = rand::rng();

        let top = 12.0;
        let dist = particle_rad;

        let mut e1 = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(0.0, top)).set_mass_2(0.0).set_phase(Phase::Solid);
        e1.body = -2;
        sim.add_particle(e1);

        let mut i = 0.0 + dist;
        while i < 4.0 * scale - dist { //for (double i = 0 + dist; i < 4*scale - dist; i += dist) {
            let mut part = *Particle::default().set_colour(red).set_radius(particle_rad).set_pos(Vec2::new(i, top)).set_mass_2( 2.0).set_phase(Phase::Solid);
            part.body = -2;
            sim.add_particle(part);

            sim.add_distance_constraint(
                DistanceConstraint::new(dist, sim.particles.len() - 2, sim.particles.len() - 1, false)
            );
            // m_globalConstraints[STANDARD].append(
            //             new DistanceConstraint(dist, m_particles.size() - 2, m_particles.size() - 1));

            i += dist;
        }

    //    Particle *e2 = new Particle(glm::dvec2(2*scale, top), 0, SOLID);
    //    e2->bod = -2;
    //    m_particles.append(e2);

        sim.add_distance_constraint(
            DistanceConstraint::new(dist, sim.particles.len() - 2, sim.particles.len() - 1, false)
        );
        // m_globalConstraints[STANDARD].append(
        //             new DistanceConstraint(dist, m_particles.size() - 2, m_particles.size() - 1));

        let mut particles2 = ParticleVec::new(); //QList<Particle *> particles;

        let start = -0.5 * scale;
        let mut x = start;
        while x < start + (1.0 * scale) { //for(double x = start; x < start + (1 * scale); x += delta) {
            let mut y = -0.5 * scale;
            while y < 0.5 * scale { //for(double y = -.5 * scale; y < .5 * scale; y += delta) {
                let r1: f32 = rng.random();
                let r2: f32 = rng.random();

                particles2.push(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x,y) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2( 1.0));
                y += delta;
            }

            x += delta;
        }

        let gas_idx = sim.create_gas(&particles2, 1.5, true);
        //GasConstraint *gs = createGas(&particles2, 1.5, true);

        sim.create_smoke_emitter(Vec2::new(0.0, 0.0), 15.0, gas_idx /*gs*/);
        // createSmokeEmitter(glm::dvec2(0,0), 15, gs);
        particles2.clear();
    }

    
    pub fn init_volcano(sim: &mut Simulation) {
        let scale = 10.0;
        let delta = 0.2;

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let red = Vec4::RED;

        let mut rng = rand::rng();

        let mut x = 1.0;
        while x <= scale { // for(double x = 1.; x <= scale; x+=delta) {
            sim.add_particle(*Particle::default().set_colour(red).set_radius(particle_rad).set_pos(Vec2::new(-x,scale-x)).set_mass_2(0.0));
            sim.add_particle(*Particle::default().set_colour(red).set_radius(particle_rad).set_pos(Vec2::new(x,scale-x)).set_mass_2(0.0));

            x += delta;
        }


        sim.x_boundaries = Vec2::new(-2.0 * scale,2.0 * scale);
        sim.y_boundaries = Vec2::new(0.0 * scale,10.0 * scale);

        let mut particles = ParticleVec::new(); //QList<Particle *> particles;

        let delta = 0.8;
        let mut y = 0.0;
        while y < scale-1.0 { //for(double y = 0.; y < scale-1.; y+=delta) {
            let mut x = 0.0;
            while x < scale - y - 1.0 { //for(double x = 0.; x < scale-y-1; x += delta) {
                let r1: f32 = rng.random();
                let r2: f32 = rng.random();
                let r3: f32 = rng.random();
                let r4: f32 = rng.random();

                particles.push(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x,y) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2(1.1));
                particles.push(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(-x,y) + 0.2 * Vec2::new(r3 - 0.5, r4 - 0.5)).set_mass_2(1.1));

                x += delta;
            }

            y += delta;
        }
        let fluid_idx = sim.create_fluid(&particles, 1.0); //TotalFluidConstraint *fs = createFluid(&particles, 1);
        particles.clear();

        sim.create_fluid_emitter(Vec2::new(0.0,0.0), scale*4.0, fluid_idx);

    //    double top = scale-.5, dist = PARTICLE_RAD;

    //    Particle *e1 = new Particle(glm::dvec2(-1-dist, top), 0, SOLID);
    //    e1->bod = -2;
    //    m_particles.append(e1);

    //    for (double i = -1; i <= 2; i += dist) {
    //        Particle *part = new Particle(glm::dvec2(i, top), 1, SOLID);
    //        part->bod = -2;
    //        m_particles.append(part);
    //        m_globalConstraints[STANDARD].append(
    //                    new DistanceConstraint(dist, m_particles.size() - 2, m_particles.size() - 1));
    //    }
    }

    pub fn init_wrecking_ball(sim: &mut Simulation) {
        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        let mut rng = rand::rng();

        sim.x_boundaries = Vec2::new(-15.0,100.0);
        sim.y_boundaries = Vec2::new(0.0,1000000.0);

        let dim = Vec2::new(6.0,2.0);
        let height = 8;
        let width = 2;
        
        let root2 = f32::sqrt(2.0);

       let mut particles = ParticleVec::new(); //QList<Particle *> vertices;
        //QList<SDFData> data;

        let mut sdf_data = Vec::<SdfData>::new();
        sdf_data.push(SdfData::new(Vec2::new(-1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(-1.0, 1.0).normalize(), particle_rad * root2));
        // data.append(SDFData(glm::normalize(glm::dvec2(-1,-1)), PARTICLE_RAD * root2));
        // data.append(SDFData(glm::normalize(glm::dvec2(-1,1)), PARTICLE_RAD * root2));

        let mut i = 0;
        while i < (dim.x as i32 - 2) { //for (int i = 0; i < dim.x - 2; i++) {
            sdf_data.push(SdfData::new(Vec2::new(0.0, -1.0).normalize(), particle_rad));
            sdf_data.push(SdfData::new(Vec2::new(0.0, 1.0).normalize(), particle_rad));
            // data.append(SDFData(glm::normalize(glm::dvec2(0,-1)), PARTICLE_RAD));
            // data.append(SDFData(glm::normalize(glm::dvec2(0,1)), PARTICLE_RAD));

            i += 1;
        }

        sdf_data.push(SdfData::new(Vec2::new(1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(1.0, 1.0).normalize(), particle_rad * root2));
        // data.append(SDFData(glm::normalize(glm::dvec2(1,-1)), PARTICLE_RAD * root2));
        // data.append(SDFData(glm::normalize(glm::dvec2(1,1)), PARTICLE_RAD * root2));

        let mut j = -width;
        while j <= width { //for (int j = -width; j <= width; j++) {

            let mut i = height - 1;
            while i >= 0 { //for (int i = height - 1; i >= 0; i--) {

                let mut x = 0;
                while x < dim.x as i32 { //for (int x = 0; x < dim.x; x++) {
                    let num = if i % 2 == 0 { 3 } else { -1 }; //(i % 2 == 0 ? 3 : -1);
                    let x_val = j as f32 * (f32::EPSILON + dim.x / 2.0) + particle_diam * (x % dim.x as i32) as f32 - num as f32 * particle_rad;

                    let mut y = 0;
                    while y < dim.y as i32 { //for (int y = 0; y < dim.y; y++) {
                        let y_val = (i as f32 * dim.y + (y % dim.y as i32) as f32 + f32::EPSILON) * particle_diam + particle_rad;
                        let mut part = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x_val, y_val)).set_mass_2(30.0);
                        part.s_friction = 1.0;
                        part.k_friction = 1.0;
                        particles.push(part);

                        y += 1;
                    }

                    x += 1;
                }
                sim.create_rigid_body(&mut particles, &sdf_data); //createRigidBody(&vertices, &data);
                particles.clear();

                i -= 1;
            }

            j += 1;
        }

        let scale = 6.0;
        let delta = 0.4;
        particles.clear(); //QList<Particle *> particles;

        let num = 1.;
        let start = sim.x_boundaries.x + 1.0;
        let mut x = start;
        while x < start + (scale / num) { //for(double x = start; x < start + (scale / num); x += delta) {
            let mut y = 0.0;
             while y < 1.2 * scale { //for(double y = 0; y < 1.2 * scale; y += delta) {
                let r1: f32 = rng.random();
                let r2: f32 = rng.random();

                particles.push(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x,y) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5)).set_mass_2(1.0));

                y += delta;
            }

            x += delta;
        }
        sim.create_fluid(&particles, 2.5);
        particles.clear();

        let idx = sim.particles.len();
        sim.add_particle(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(10.0, 50.0)).set_mass_2(0.0));
        sdf_data.clear();

        let base = Vec2::new(57.0, 50.0);
        particles.push(*Particle::default().set_radius(particle_rad).set_pos(base).set_mass_2(1000.0));
        let mut a: f32 = 0.0;
        while a <= 360.0 { // for (double a = 0; a <= 360; a+=30) {
            let vec = Vec2::new(f32::cos(a.to_radians() /*D2R(a)*/), f32::sin(a.to_radians() /*D2R(a)*/));
            particles.push(*Particle::default().set_radius(particle_rad).set_pos(vec * particle_rad + base).set_mass_2(1000.0));
            sdf_data.push(SdfData::new(vec, particle_rad * 1.5));
        
            a+=30.0;
        }
        sdf_data.push(SdfData::new(Vec2::new(0.0, 0.0), 0.0));
        sim.create_rigid_body(&mut particles, &sdf_data);

        //m_globalConstraints[STANDARD].append(new DistanceConstraint(idx, idx + 1, &m_particles));
        sim.add_distance_constraint(DistanceConstraint::from_particles(idx, idx + 1, &sim.particles));
            

    }
}