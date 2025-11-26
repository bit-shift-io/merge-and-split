use rand::Rng;

use crate::{core::math::vec2::Vec2, simulation::{constraints::total_fluid_constraint::{TotalFluidConstraint, TotalFluidConstraintVec}, particles::{particle::{Particle, Phase}, particle_vec::ParticleVec}}};

pub struct FluidEmitter {
    posn: Vec2,
    particles_per_sec: f32,
    timer: f32,
    totalTimer: f32,
    fluid_index: usize, // TotalFluidConstraint *m_fs;
    grains: ParticleVec,
    // glm::dvec2 m_posn;
    // double m_particles_per_sec;
    // double timer;
    // double totalTimer;
    // TotalFluidConstraint *m_fs;
    // QList<Particle *> grains;
}

impl FluidEmitter {
    pub fn new(posn: Vec2, particles_per_sec: f32, fluid_index: usize /*TotalFluidConstraint *fs*/) -> Self {
        Self {
            posn,
            particles_per_sec,
            timer: 0.0,
            totalTimer: 0.0,
            fluid_index,
            grains: ParticleVec::new(),
        }
    }


    pub fn tick(&mut self, estimates: &mut ParticleVec, secs: f32, global_standard_total_fluid_constraints: &mut TotalFluidConstraintVec) { 
        //let mut i = global_standard_total_fluid_constraints[self.fluid_index].ps.len()-1;
        for i in (0..global_standard_total_fluid_constraints[self.fluid_index].ps.len()).rev() { //while i >= 0 { //for(int i = global_standard_total_fluid_constraints[self.fluid_index].ps.len()-1; i >= 0; i--) {
            let eidx = global_standard_total_fluid_constraints[self.fluid_index].ps[i];
            let p = estimates[eidx]; //Particle *p = estimates->at(m_fs->ps.at(i));
                //            std::cout << p << std::endl;
    //            double lambda = m_fs->lambdas[i];
                //            std::cout << lambda << std::endl;
            //            if(lambda >= -.1 && glm::length(p->v) < .05 && glm::length(p->p - p->ep) < .05) {
            //            if(p->p.y >= 10 || fabs(p->p.x) >= 10 ) {
            if p.vel.magnitude() < 0.06 && p.pos.y <= 5.0 { //(glm::length(p->v) < .06 && p->p.y <= 5) {

                let lambdas_i = match global_standard_total_fluid_constraints[self.fluid_index].lambdas.get(&i) {
                    Some(value) => *value,
                    None => 0.0
                };

                if lambdas_i <= 0.0 { // if(m_fs->lambdas[i] <= 0) {
                    estimates[eidx].t -= 1.0; //p->t -= 1;
                    if estimates[eidx].t <= 0.0 { //(p->t <= 0) {
                        estimates[eidx].t = 0.0; //p->t = 0;

                        //                p->ph = SOLID;
                        //                Particle *newP = new Particle(p->p, 0, SOLID);
                        //                newP->v = p->v;
                        //            p->imass -= secs;
                        //            if(p->imass == 0)

                        estimates[eidx].imass = 0.0; //p->imass = 0;
                        estimates[eidx].phase = Phase::Solid; //p->ph = SOLID;
                        estimates[eidx].pos_guess = estimates[eidx].pos; //p->ep = p->p;
                        estimates[eidx].vel = Vec2::new(0.0, 0.0); // p->v = glm::dvec2();
                        estimates[eidx].force = Vec2::new(0.0, 0.0);// p->f = glm::dvec2();
                        //                grains.append(newP);
                        //                estimates->append(newP);
                        //                estimates->removeAt(m_fs->ps.at(i));
                        //                if(m_fs->ps.contains(i))
                        global_standard_total_fluid_constraints[self.fluid_index].remove_particle(i); // m_fs->removeParticle(i);
                        //                delete p;
                        //                p->imass = 1;
                        //                p->ph = SOLID;
                        //                m_fs->ps.removeAt(i);
                    }
                } else {
                    estimates[eidx].t += secs; //p->t += secs;
                    if estimates[eidx].t > 3.0 { //if(p->t > 3) {
                        estimates[eidx].t = 3.0; //p->t = 3;
                    }
                }
                //        }
            }

            //i -= 1;
        }


        //    for(int i=0; i<grains.size(); i++) {
        //        Particle *p = grains.at(i);
    //        if(glm::length(p->v) <= .02) {
    //            grains.removeAt(i);
    //            p->imass = 0;
    //            p->v = glm::dvec2();
    //            p->f = glm::dvec2();
    //            p->ep = p->p;
    //        }
    //    }

        self.timer += secs;
        self.totalTimer += secs;

        while self.totalTimer < 5.0 && self.timer >= 1.0/self.particles_per_sec {
            self.timer -= 1.0/self.particles_per_sec;
            if self.fluid_index != usize::MAX { // if (m_fs != NULL) {

                let particle_diam = 0.5;
                let particle_rad = particle_diam / 2.0;

                let mut p = *Particle::default().set_radius(particle_rad).set_pos(self.posn).set_mass_2(1.0).set_phase(Phase::Fluid);

                // todo: do not use rand here! want something deterministic
                let mut rng = rand::rng();
                let r1: f32 = rng.random();

                p.vel = Vec2::new(r1, 1.0);
                global_standard_total_fluid_constraints[self.fluid_index].add_particle(estimates.len());
                estimates.push(p);
            }
        }
    }
}