use rand::Rng;

use crate::{
    core::math::{vec2::Vec2, vec4::Vec4},
    game::level::{level_builder::LevelBuilderContext, level_builder_operation::LevelBuilderOperation},
    simulation::{
        constraints::distance_constraint::DistanceConstraint,
        particles::{particle::Particle, particle_vec::ParticleVec, shape_builder::{adjacent_sticks::AdjacentSticks, circle::{Circle, SpaceDistribution}, shape_builder::ShapeBuilder}, simulation::Simulation},
    },
};

pub struct WaterBalloonDrop {}

impl WaterBalloonDrop {
    /// Creates a filled water balloon at the specified position
    /// 
    /// # Parameters
    /// - `sim`: The simulation to add particles to
    /// - `center`: Center position of the balloon
    /// - `particle_rad`: Radius of individual particles
    /// - `balloon_radius`: Radius of the balloon membrane
    /// - `samples`: Number of particles in each membrane circle (fewer = less particles)
    fn create_water_balloon(
        sim: &mut Simulation,
        center: Vec2,
        particle_rad: f32,
        balloon_radius: f32
    ) {
        // Membrane
        // may need to do some overlap to stop fluid leaking out?
        let mut part = *Particle::default()
                .set_colour(Vec4::BLUE)
                .set_radius(particle_rad)
                //.set_pos(Vec2::new(f32::sin(angle), f32::cos(angle)) * balloon_radius + center)
                .set_mass(1.0);
            part.body = -2; // Prevent self-collision

        let mut sb = ShapeBuilder::from_particle_template(part);
        sb.apply_operation(Circle::new(center, balloon_radius, SpaceDistribution::AdjustRadius))
            .create_in_simulation(sim);

        AdjacentSticks::new(1, true)
                .apply_to_particle_handles(sim, &sb.particle_handles);
        
        // Fill with fluid particles - a grid of them that fits inside the membrane
        let delta = 2.0 * particle_rad;
        let mut particles = ParticleVec::new();
        let mut rng = rand::rng();
        let distance = balloon_radius * 0.60;
        let mut x = -distance;
        while x <= distance {
            let mut y = -distance;
            while y <= distance {
                let r1: f32 = rng.random();
                let r2: f32 = rng.random();

                particles.push(*Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x, y) + 0.2 * Vec2::new(r1 - 0.5, r2 - 0.5) + center).set_mass(1.0));
                y += delta;
            }
            x += delta;
        }

        println!("water ballon has {} particles", particles.len());

        // Higher density lets particles get closer together
        sim.create_fluid(&particles, 4.0);
    }
}

impl LevelBuilderOperation for WaterBalloonDrop {
    fn type_name(&self) -> &str {
        "WaterBalloonDrop"
    }

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
        Box::new(WaterBalloonDrop {})
    }

    fn default_spawn_chance(&self) -> f32 {
        0.3
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        let rng = &mut level_builder_context.rng;
        let particle_rad = level_builder_context.particle_template.radius;
        
        // Randomize balloon size (smaller than original)
        let balloon_radius = rng.random_range(0.5..=1.0);
        
        // Randomize position offset from cursor
        let x_offset = rng.random_range(-1.0..=1.0);
        let y_offset = rng.random_range(2.0..=2.0);
        
        let balloon_center = level_builder_context.cursor + Vec2::new(x_offset, y_offset);
        
        // Create first balloon
        Self::create_water_balloon(
            level_builder_context.sim,
            balloon_center,
            particle_rad,
            balloon_radius
        );
        
        // // 50% chance to spawn a second balloon
        // if rng.random::<f32>() < 0.5 {
        //     let second_balloon_radius = rng.random_range(1.5..=2.5);
        //     let second_samples = rng.random_range(30..=40);
        //     let second_x_offset = rng.random_range(-1.0..=1.0);
        //     let second_y_offset = rng.random_range(6.0..=10.0);
            
        //     let second_balloon_center = level_builder_context.cursor + Vec2::new(second_x_offset, second_y_offset);
            
        //     Self::create_water_balloon(
        //         level_builder_context.sim,
        //         second_balloon_center,
        //         particle_rad,
        //         second_balloon_radius,
        //         second_samples,
        //     );
        // }

        // Don't move the cursor - the balloon is above the level
    }
}
