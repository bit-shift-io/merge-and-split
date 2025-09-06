use crate::particle_system::ParticleSystem;

pub struct OperationContext {

}

// https://github.com/bit-shift-io/rust-verlet/blob/main/src/level/level_builder_operation.rs
pub trait Operation {
    fn execute(&self, ps: &mut ParticleSystem);
}