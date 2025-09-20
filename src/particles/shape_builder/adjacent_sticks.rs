//use crate::v4::{constraints::{constraint::Constraint, stick_constraint::StickConstraint}, particle_handle::{self, ParticleHandle}};

use super::{circle::Circle, shape_builder::{ShapeBuilder, ShapeBuilderOperation}};

/// Takes a Circle and created stick constraints in a grid layout between them
pub struct AdjacentSticks {
    constraint_template: StickConstraint,
    stride: usize,
    wrap_around: bool
}

impl AdjacentSticks {
    pub fn new(constraint_template: StickConstraint, stride: usize, wrap_around: bool) -> Self {
        Self {
            constraint_template,
            stride,
            wrap_around
        }
    }

    fn add_constraint_to_shape_builder_from_particle_handles(&self, shape_builder: &mut ShapeBuilder, particle_handles: [ParticleHandle; 2]) {
        let particle_a = shape_builder.particles[particle_handles[0].id()];
        let particle_b = shape_builder.particles[particle_handles[1].id()];
        let length = (particle_b.pos - particle_a.pos).length();
        let constraint = self.constraint_template.clone().set_particle_handles(particle_handles).set_length(length).box_clone();
        shape_builder.add_constraint(constraint);
    }
}

impl ShapeBuilderOperation for AdjacentSticks {
    fn apply_to_shape_builder(&self, shape_builder: &mut ShapeBuilder) {
        let radius = shape_builder.particle_radius();

        let particle_count = shape_builder.particles.len();

        for pi in 0..particle_count {
            let mut pi_next = pi + self.stride;
            if pi_next >= particle_count {
                if !self.wrap_around {
                    continue;
                }

                pi_next -= particle_count;
            }

            let particle_handles = [
                ParticleHandle::new(pi),
                ParticleHandle::new(pi_next)
            ];
            self.add_constraint_to_shape_builder_from_particle_handles(shape_builder, particle_handles);
        }
      
    }
}