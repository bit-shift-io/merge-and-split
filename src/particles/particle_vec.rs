use std::{ops::{Deref, DerefMut, Index, IndexMut, RangeBounds}, ptr::NonNull, slice::{Iter, IterMut}, vec::ExtractIf};

use crate::particles::particle::Particle;


//pub struct ParticleHandle(pub usize);
pub type ParticleHandle = usize;


pub struct ParticleVec(pub Vec<Particle>);

impl<const N: usize> From<[Particle; N]> for ParticleVec {
    fn from(s: [Particle; N]) -> Self {
        let mut se = Self(Vec::<Particle>::from(s));
        
        // Update indicies of newly added particles.
        for i in 0..se.0.len() {
            se[i].set_index(i);
        }
        return se;
    }
}

impl ParticleVec {
    pub fn new() -> Self {
        Self {
            0: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn as_slice(&self) -> &[Particle] {
        self.0.as_slice()
    }

    pub fn get_subslice(&self, range: std::ops::RangeFrom<usize>) -> Option<&[Particle]> {
        self.0.get(range)
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, value: Particle) {
        self.0.push(*value.clone().set_index(self.0.len()));
    }

    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
    }

    pub fn extend(&mut self, other_vec: &ParticleVec) {
        let index_start = self.0.len();
        self.0.extend(other_vec.0.clone()); // Is there a non-clone way to do this?

        // Update indicies of newly added particles.
        for i in index_start..self.0.len() {
            self.0[i].set_index(i);
        }
    }

    pub fn iter(&self) -> Iter<'_, Particle> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Particle> {
        self.0.iter_mut()
    }

    // pub fn extract_if<F, R>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, A>
    // where
    //     T: Particle,
    //     F: FnMut(&mut T) -> bool,
    //     R: RangeBounds<usize>,
    // {
    //     self.0.extract_if(range, filter)
    // }
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

// impl Default for ParticleVec {
//     fn default() -> Self {
//         Self {
//             0: vec![],
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use crate::math::vec2::Vec2;
    use super::*;

    // #[test]
    // fn default() {
    //     let ps = ParticleVec::default();
    //     assert_eq!(ps.0, vec![]);
    // }

    #[test]
    fn new() {
        let ps = ParticleVec::new();
        assert_eq!(ps.0, vec![]);
    }

    #[test]
    fn from_constructor() {
        let ps = ParticleVec::from([Particle::default(), Particle::default()]);
        assert_eq!(ps.len(), 2);
    }
}