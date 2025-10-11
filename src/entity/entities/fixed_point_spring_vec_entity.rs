use crate::{constraints::fixed_point_spring::FixedPointSpringVec, entity::entity::{Entity, UpdateContext}};


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
}