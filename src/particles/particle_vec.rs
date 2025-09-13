use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::particles::particle::Particle;


pub struct ParticleVec(Vec<Particle>);

impl ParticleVec {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, value: Particle) {
        self.0.push(value);
    }

    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
    }
}

// Implement the Index trait for immutable access
impl Index<usize> for ParticleVec {
    type Output = Particle;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index] // Access the inner Vec and then its element
    }
}

// Implement the IndexMut trait for mutable access
impl IndexMut<usize> for ParticleVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index] // Access the inner Vec and then its mutable element
    }
}

impl Default for ParticleVec {
    fn default() -> Self {
        Self {
            0: vec![],
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::math::Vec2;
    use super::*;

    #[test]
    fn default() {
        let ps = ParticleVec::default();
        assert_eq!(ps.0, vec![]);
    }
}