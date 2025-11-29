use crate::core::math::aabb2d::Aabb2d;

pub struct FinishEntity {
    pub aabb: Aabb2d,
}

impl FinishEntity {
    pub fn new(aabb: Aabb2d) -> Self {
        Self {
            aabb,
        }
    }
}

pub struct FinishEntitySystem {
    pub entities: Vec<FinishEntity>,
}

impl FinishEntitySystem {
    pub fn new() -> Self {
        Self {
            entities: vec![],
        }
    }

    pub fn push(&mut self, entity: FinishEntity) {
        self.entities.push(entity);
    }
}
