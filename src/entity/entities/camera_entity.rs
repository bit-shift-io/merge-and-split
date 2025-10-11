use crate::{constraints::fixed_point_spring::FixedPointSpringVec, entity::entity::{Entity, UpdateContext}};


pub struct CameraEntity {
}

impl CameraEntity {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Entity for CameraEntity {
    fn update(&mut self, context: &mut UpdateContext) {
        // todo:
    }
}