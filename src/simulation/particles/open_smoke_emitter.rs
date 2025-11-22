use std::f32::consts::PI;

use crate::{core::math::vec2::Vec2, simulation::{constraints::gas_constraint::GasConstraint, particles::{particle::{Particle, Phase}, particle_vec::ParticleVec}}};

pub struct OpenSmokeEmitter {

    pub posn: Vec2,
    pub particles_per_sec: f32,
    pub particles: ParticleVec,
    pub timer: f32,
    pub gas_index: usize, // GasConstraint *m_gs;
}

impl OpenSmokeEmitter {
    pub fn new(posn: Vec2, particles_per_sec: f32, gas_index: usize /*GasConstraint *gs*/) -> Self {
        Self {
            posn,
            particles_per_sec,
            particles: ParticleVec::new(),
            timer: 0.0,
            gas_index
        }
    }

    pub fn tick(&mut self, estimates: &mut ParticleVec, secs: f32, global_standard_gas_constraints: &mut Vec<GasConstraint>) {
        self.timer += secs;

        let particle_diam = 0.5;
        let particle_rad = particle_diam / 2.0;

        while self.timer >= 1./self.particles_per_sec {
            self.timer -= 1./self.particles_per_sec;

            let mut p = *Particle::default().set_radius(particle_rad).set_pos(self.posn).set_mass_2(0.1).set_phase(Phase::Gas);
            self.particles.push(p);
            if self.gas_index != usize::MAX { //m_gs != NULL) {
                p = *Particle::default().set_radius(particle_rad).set_pos(self.posn).set_mass_2(1.0).set_phase(Phase::Gas);
                global_standard_gas_constraints[self.gas_index].add_particle(p, estimates.len()); //m_gs->addParticle(p, estimates->size());
                estimates.push(p);
            }
        }

        for i in 0..self.particles.len() { //for(Particle *p: m_particles) {
            let mut p = self.particles[i];
            if p.phase == Phase::Fluid || p.phase == Phase::Gas { //p->ph == FLUID || p->ph == GAS) {
                p.vel = Vec2::new(0.0, 0.0);
                let mut sum = 0.0;

                for k in 0..estimates.len() {
                    let n = estimates[k];
                    let r = p.pos - n.pos;
                    let p6 = poly6(r.dot(r));
                    p.vel += n.vel * p6;
                    sum += p6;
                }

                if sum > 0.0 {
                    p.pos += p.vel * secs / sum;
                }
            }
        }
    }
}


const h: f32 = 2.0;
const h2: f32 = 4.0;
const h6: f32 = 64.0;
const h9: f32 = 512.0;


pub fn poly6(r2: f32) -> f32 {
    if r2 >= h2 {
        return 0.0;
    }
    let term2 = (h2 - r2);
    return (315.0 / (64. * PI * h9)) * (term2 * term2 * term2);
//    return (H-r) / (H*H);
}