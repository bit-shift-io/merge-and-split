//use bevy::core::math::{bounding::Aabb2d, Vec2};

//use crate::v5::{particle::Particle, particle_handle::ParticleHandle, particle_system::ParticleSystem, particle_vec::SharedParticleVec};

use cgmath::{Bounded, InnerSpace};

use crate::{core::math::{aabb2d::Aabb2d, vec2::{Vec2, vec2_max, vec2_min}}, simulation::particles::{particle::Particle, particle_vec::{ParticleHandle, ParticleVec}, simulation::Simulation}};



// Utility function that takes 2 points (a line segment) and a radius
// and calculates how many circles can fit touching each other between the 2 points.
pub fn radius_divisions_between_points(p1: Vec2, p2: Vec2, radius: f32) -> usize {
    let dist = (p2 - p1).magnitude();
    let divisions = (dist / (radius * 2.0)) as usize;
    return divisions;
}

pub trait ShapeBuilderOperation {
    fn apply_to_shape_builder(&self, shape_builder: &mut ShapeBuilder);
}

pub struct ShapeBuilder {
    pub particles: ParticleVec,
    pub particle_template: Particle,

    /* 
    pub constraints: Vec<Box<dyn Constraint + Send + Sync>>,
    */

    pub cursor: Vec2,
    /* 
    sticks: Vec<StickPrim>,
    springs: Vec<SpringPrim>,

    // particle properties
    is_static: bool,
    mass: f32,
    color: Color,
    radius: f32,
    stiffness_factor: f32,

    // spring properties
    spring_constant: f32,
    elastic_limit: f32,
    damping: f32,
*/
    pub particle_handles: Vec<ParticleHandle>,
    //pub stick_handles: Vec<StickHandle>,

    /* 
    pub constraint_handles: Vec<ConstraintHandle>,
    */
}

impl ShapeBuilder {
    pub fn new() -> Self {
        Self { 
            particles: ParticleVec::new(), 
            particle_template: Particle::default(),
/* 
            constraints: vec![],
            */
            //constraint_template: Box::new(StickConstraint::default()),

            cursor: Vec2::new(0.0, 0.0),

            particle_handles: vec![],

            /* 
            constraint_handles: vec![],
*/

            //stick_handles: vec![],
            //spring_handles: vec![],
        }    
    }

    pub fn from_shape_builder_templates(sb: &ShapeBuilder) -> Self {
        let mut new_sb = ShapeBuilder::new();
        new_sb.set_particle_template(sb.particle_template);

        //new_sb.set_constraint_template(sb.constraint_template.as_ref().clone());
        //new_sb.constraint_template = sb.constraint_template.box_clone(); //Box::new(*sb.constraint_template.as_ref());
        
        new_sb
    }

    /* 
    pub fn add_constraint(&mut self, constraint: Box<dyn Constraint + Send + Sync>) -> &mut Self {
        self.constraints.push(constraint);
        self
    }*/

    pub fn set_particle_template(&mut self, particle_template: Particle) -> &mut Self {
        self.particle_template = particle_template;
        self
    }

    pub fn add_particle(&mut self, particle: Particle) -> &mut Self {
        self.particles.push(particle);
        self
    }

    // create a particle from the particle_template and place it at the given position
    // then add it
    pub fn add_particle_at_position(&mut self, pos: Vec2) -> &mut Self {
        let p = self.create_particle().set_pos(pos).clone();
        self.add_particle(p);
        self
    }

    // create a particle from the particle_template
    pub fn create_particle(&self) -> Particle {
        self.particle_template.clone()
    }

    // pub fn create_in_shared_particle_vec(&mut self, shared_particle_vec: &SharedParticleVec) -> &mut Self {
    //     let mut particle_vec = shared_particle_vec.as_ref().write().unwrap();
    //     let mut particle_handles = particle_vec.add_vec(&self.particles);
    //     self.particle_handles.append(&mut particle_handles);
    //     self
    // }

    pub fn create_in_particle_vec(&mut self, particle_vec: &mut ParticleVec) -> &mut Self {
        let start_index = particle_vec.len();
        (*particle_vec).extend(&self.particles);

        for i in start_index..particle_vec.len() {
            self.particle_handles.push(i);
        }
        // let mut particle_handles = (*particle_vec).extend(&self.particles);
        // self.particle_handles.append(&mut particle_handles);
        self
    }

    pub fn create_in_simulation(&mut self, sim: &mut Simulation) -> &mut Self {
        self.create_in_particle_vec(&mut sim.particles);
        self
    }
/* 
    fn create_in_particle_container(&mut self, particle_container: &mut ParticleContainer) -> &mut Self {
        for particle in self.particles.iter() {
            let particle_handle = particle_container.add(*particle);
            self.particle_handles.push(particle_handle);
        }
        self
    }

    fn create_in_constraint_container(&mut self, constraint_container: &mut ConstraintContainer, particle_handle_offset: u64) -> &mut Self {
        for constraint in self.constraints.iter() {
            let mut constraint = constraint.box_clone();
            constraint.offset_particle_handles(particle_handle_offset);
            let constraint_handle = constraint_container.add(constraint);
            self.constraint_handles.push(constraint_handle);
        }
        self
    }

    pub fn create_in_particle_sim(&mut self, particle_sim: &mut ParticleSim) -> &mut Self {
        let mut particle_container = particle_sim.particle_container.as_ref().write().unwrap();
        let mut constraint_container = particle_sim.constraint_container.as_ref().write().unwrap();

        let particle_handle_offset = particle_container.particles.len() as u64;

        self.create_in_particle_container(&mut particle_container);
        self.create_in_constraint_container(&mut constraint_container, particle_handle_offset);

        self
    }
*/
    pub fn particle_radius(&self) -> f32 {
        self.particle_template.radius
    }

    pub fn apply_operation<T: ShapeBuilderOperation>(&mut self, operation: T) -> &mut Self {
        operation.apply_to_shape_builder(self);
        self
    }

    pub fn add_particles_from_points(&mut self, points: &Vec<Vec2>) -> &mut Self {
        for p in points {
            self.add_particle_at_position(p.clone());
        }
        self
    }

    pub fn get_aabb(&self) -> Aabb2d {
        // extracted from Aabb2d::from_point_cloud
        let mut points_iter = self.particles.iter().map(|particle| particle.pos);//.collect::<Vec<Vec2>>();//.try_into().unwrap();
        
        let first = points_iter
            .next()
            .expect("point cloud must contain at least one point for Aabb2d construction");

        let (min, max) = points_iter.fold((first, first), |(prev_min, prev_max), point| {
            (vec2_min(point, prev_min), vec2_max(point, prev_max))
        });

        Aabb2d {
            min,
            max
        }
    }

    // pub fn extract_left_most_particles(&mut self) -> ShapeBuilder {
    //     let aabb = self.get_aabb();

    //     let mut s = ShapeBuilder::from_shape_builder_templates(&self);

    //     let r = self.particles.extract_if(|particle| particle.pos.x == aabb.min.x).collect::<Vec<_>>();
    //     //s.add_particles(r); // todo: add a fn for this
    //     for particle in r {
    //         s.add_particle(particle);
    //     }
    //     s
    // }
}
