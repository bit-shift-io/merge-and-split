use crate::{entity::entity::{Entity, EntityConstraintSolver, UpdateContext}, particles::{particle_vec::ParticleVec, simulation::Simulation}, platform::camera::Camera};
use winit::{keyboard::KeyCode};

pub struct EntitySystem {
    pub entities: Vec<Box<dyn Entity>>,
    pub constraint_solvers: Vec<Box<dyn EntityConstraintSolver>>,
}

impl EntitySystem {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            constraint_solvers: vec![],
        }
    }

    pub fn push<T: Entity + 'static>(&mut self, entity: T) -> &mut Self {
        self.entities.push(Box::new(entity));
        self
    }

    pub fn add_constraint_solver<T: EntityConstraintSolver + 'static>(&mut self, entity: T) -> &mut Self {
        self.constraint_solvers.push(Box::new(entity));
        self
    }

    pub fn solve_constraints(&mut self, sim: &mut Simulation, time_delta: f32) {
        for c in self.constraint_solvers.iter_mut() {
            c.solve_constraints(sim, time_delta);
        }
    }

    pub fn update(&mut self, particle_vec: &mut ParticleVec, sim: &mut Simulation, camera: &mut Camera, time_delta: f32) {
        let mut context = UpdateContext {
            time_delta,
            particle_vec,
            sim,
            camera,
            //level: self,
        };
        for entity in self.entities.iter_mut() {
            entity.update(&mut context);
        }
    }

    // Gross having to call this on each entity. Should use some subscribe/listener or traits instead
    pub fn handle_key(&mut self, key: KeyCode, pressed: bool) {
        for entity in self.entities.iter_mut() {
            entity.handle_key(key, pressed);
        }
    }
}