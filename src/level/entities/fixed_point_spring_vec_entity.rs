use crate::{constraints::fixed_point_spring::FixedPointSpringVec, level::level_entity::{LevelEntity, UpdateContext}};

pub struct FixedPointSpringVecEntity {
    pub fixed_point_spring_vec: FixedPointSpringVec,
}

impl LevelEntity for FixedPointSpringVecEntity {
    fn update(&mut self, context: &mut UpdateContext) {
        self.fixed_point_spring_vec.execute(context.particle_vec, context.time_delta);
    }
}