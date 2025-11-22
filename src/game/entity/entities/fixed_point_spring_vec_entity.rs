use winit::keyboard::KeyCode;

use crate::{game::entity::entity::{Entity, UpdateContext}, simulation::constraints::fixed_point_spring::FixedPointSpringVec};

pub struct FixedPointSpringVecEntity {
    pub fixed_point_spring_vec: FixedPointSpringVec,
}

impl FixedPointSpringVecEntity {
    pub fn new(fixed_point_spring_vec: FixedPointSpringVec) -> Self {
        Self {
            fixed_point_spring_vec
        }
    }
}

impl Entity for FixedPointSpringVecEntity {
    fn update(&mut self, context: &mut UpdateContext) {
        self.fixed_point_spring_vec.execute(context.particle_vec, context.time_delta);
    }

    // ughly!
    fn handle_key(&mut self, key: KeyCode, is_pressed: bool) -> bool { false }
}