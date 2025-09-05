use crate::particle::Particle;



pub struct ParticleSystem {
    pub particles: Vec<Particle>,

}

impl ParticleSystem {
    pub fn len(&self) -> usize {
        self.particles.len()
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self {
            particles: vec![],
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::math::Vec2;
    use super::*;

    #[test]
    fn default() {
        let ps = ParticleSystem::default();
        assert_eq!(ps.particles, vec![]);
    }
}