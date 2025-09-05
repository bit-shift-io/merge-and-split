use crate::particle::Particle;



pub struct ParticleSystem {
    pub particles: Vec<Particle>,

}

impl ParticleSystem {

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
    use super::*;

    #[test]
    fn default() {
        let ps = ParticleSystem::default();
        assert_eq!(ps.particles, vec![]);
    }


}