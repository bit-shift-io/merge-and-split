use cgmath::InnerSpace;

use crate::{constraints::stick::{Stick, StickVec}, particles::particle_vec::{ParticleHandle, ParticleVec}};

use super::{circle::Circle, shape_builder::{ShapeBuilder, ShapeBuilderOperation}};

/// Takes a Circle and created stick constraints in a grid layout between them
pub struct AdjacentSticks {
    constraint_template: Stick,
    stride: usize,
    wrap_around: bool
}

impl AdjacentSticks {
    pub fn new(constraint_template: Stick, stride: usize, wrap_around: bool) -> Self {
        debug_assert!(stride > 0); // stride of zero would be bad as it would create a stick between a particle and itself
        Self {
            constraint_template,
            stride,
            wrap_around
        }
    }

    pub fn apply_to_particle_handles(&self, particle_vec: &ParticleVec, particle_handles: &Vec<ParticleHandle>, stick_vec: &mut StickVec) {
        let radius = particle_vec[particle_handles[0]].radius; //shape_builder.particle_radius();
        let particle_count = particle_handles.len(); //shape_builder.particles.len();

        for pi in 0..particle_count {
            let mut pi_next = pi + self.stride;
            if pi_next >= particle_count {
                if !self.wrap_around {
                    continue;
                }

                pi_next -= particle_count;
            }

            let particle_handles = [
                particle_handles[pi],
                particle_handles[pi_next]
            ];

            let particle_a = particle_vec[particle_handles[0]]; // shape_builder.particles[particle_handles[0]];
            let particle_b = particle_vec[particle_handles[1]]; //shape_builder.particles[particle_handles[1]];
            let length = (particle_b.pos - particle_a.pos).magnitude();
            let mut stick = self.constraint_template.clone();
            stick.set_particle_handles(particle_handles).set_length(length);
            stick_vec.push(stick);
        }
    }
}

// impl ShapeBuilderOperation for AdjacentSticks {
//     fn apply_to_shape_builder(&self, shape_builder: &mut ShapeBuilder) {
//         let radius = shape_builder.particle_radius();

//         let particle_count = shape_builder.particles.len();

//         for pi in 0..particle_count {
//             let mut pi_next = pi + self.stride;
//             if pi_next >= particle_count {
//                 if !self.wrap_around {
//                     continue;
//                 }

//                 pi_next -= particle_count;
//             }

//             let particle_handles = [
//                 pi,
//                 pi_next
//             ];
//             self.add_constraint_to_shape_builder_from_particle_handles(shape_builder, particle_handles);
//         }
      
//     }
// }