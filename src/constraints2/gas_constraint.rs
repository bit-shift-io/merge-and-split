use std::collections::HashMap;

use cgmath::InnerSpace;
use std::f32::consts::PI;

use crate::{math::{vec2::Vec2, vec3::Vec3}, particles::{body::Body, particle::Phase, particle_vec::ParticleVec}};

const h: f32 = 2.0;
const h2: f32 = 4.0;
const h6: f32 = 64.0;
const h9: f32 = 512.0;

const s_solid: f32 = 0.5; // Fluid-solid coupling constant
const relaxation: f32 = 0.01; // Epsilon in gamma correction denominator

// Pressure terms
const k_p: f32 = 0.1;
const e_p: f32 = 4.0;
const dq_p: f32 = 0.2;


pub struct GasConstraint {
    pub p0: f32,
    pub neighbors: Vec<Vec<usize>>,
    pub ps: Vec<usize>,
    pub deltas: Vec<Vec2>,
    pub lambdas: HashMap<usize, f32>,
    pub open: bool,
}

impl GasConstraint {
    pub fn new(density: f32, particles: &Vec<usize>, open: bool) -> Self {
        let mut neighbors = Vec::<Vec<usize>>::new(); //[particles->size()];
        let mut deltas = Vec::<Vec2>::new(); //new glm::dvec2[particles->size()];

        let num_particles = particles.len();

        let mut ps= Vec::<usize>::new();
        for i in 0..num_particles { //for (int i = 0; i < particles->size(); i++) {
            ps.push(particles[i]);
            neighbors.push(Vec::<usize>::new());
            deltas.push(Vec2::new(0.0, 0.0));
        }

        Self {
            p0: density,
            neighbors,
            ps,
            deltas,
            lambdas: HashMap::new(),
            open,
        }
    }

    pub fn project(&mut self, estimates: &mut ParticleVec, counts: &Vec<usize>) {
        // Find neighboring particles and estimate pi for each particle
        self.lambdas.clear();
        for k in 0..self.ps.len() { //for (int k = 0; k < ps.size(); k++) {
            self.neighbors[k].clear();
            let i = self.ps[k];
            let mut p_i = estimates[i]; // todo: make ref?
            let mut pi = 0.0;
            let mut denom = 0.0;

            // Find neighbors
            for j in 0..estimates.len() { //(int j = 0; j < estimates->size(); j++) {

                // Check if the next particle is actually this particle
                if j != i {
                    let p_j = estimates[j]; // todo: make ref?

                    // Ignore fixed particles
                    if p_j.imass == 0.0 {
                        continue;
                    }
                    let r = p_i.pos_guess - p_j.pos_guess;
                    let rlen2 = r.dot(r); //glm::dot(r, r);
                    if rlen2 < h2 {

                        // Found a neighbor! Remember it and add to pi and the gamma denominator
                        self.neighbors[k].push(j);
                        let mut incr = poly6(rlen2) / p_j.imass;
                        if p_j.phase == Phase::Solid {
                            incr *= s_solid;
                        }
                        pi += incr;

                        let gr = self.grad(estimates, k, j);
                        denom += gr.dot(gr); //glm::dot(gr, gr);
                    }

                // If it is, cut to the chase
                } else {
                    self.neighbors[k].push(j);
                    pi += poly6(0.0) / p_i.imass;
                }
            }

            let gr = self.grad(estimates, k, i);
            denom += gr.dot(gr); //glm::dot(gr, gr);

            // Compute the gamma value
    //        cout << i << " estimated " << pi << endl;

            // Very similar to TotalFluidConstraint except we add this bit in (todo: share code better):
            let p_rat = (pi/self.p0);
            if self.open { 
                p_i.force += p_i.vel * (1.0-p_rat) * -50.0;
                estimates[i].force = p_i.force; // sync the copy with the real particle
            }

            let lambda = -((pi / self.p0) - 1.) / (denom + relaxation);
            self.lambdas.insert(i, lambda); //self.lambdas[i] = lambda;
        }

        // Compute actual deltas
        for k in 0..self.ps.len() { //(int k = 0; k < ps.size(); k++) {
            let mut delta = Vec2::new(0.0, 0.0);
            let mut f_vort = Vec2::new(0.0,0.0);
            let i = self.ps[k];
            let mut p_i = estimates[i]; // todo: make ref?

            for x in 0..self.neighbors[k].len() { // (int x = 0; x < neighbors[k].size(); x++) {
                let j = self.neighbors[k][x];
                if i == j {
                    continue;
                }
                let p_j = estimates[j]; // todo: make ref
                let r = p_i.pos_guess - p_j.pos_guess;
                let rlen = r.magnitude(); //glm::length(r);
                let sg = spikyGrad(&r, rlen);
                let lambdaCorr = -k_p * (poly6(rlen * rlen) / poly6(dq_p * dq_p * h * h)).powf(e_p); //pow(, e_p);

                let lambdas_i = match self.lambdas.get(&i) {
                    Some(value) => *value,
                    None => 0.0
                };
                let lambdas_j = match self.lambdas.get(&j) {
                    Some(value) => *value,
                    None => 0.0
                };

                delta += (lambdas_i + lambdas_j + lambdaCorr) * sg;

                // vorticity
                let gradient = spikyGrad(&r, r.dot(r)); //glm::dot(r,r));
                let w = Vec2::new(gradient.x * p_j.vel.x, gradient.y * p_j.vel.y); //gradient * p_j.vel; - is this correct? glm::dvec2(a, b) * glm::dvec2(c, d) would result in glm::dvec2(a*c, b*d)
                let cross = Vec3::new(0.0,0.0,w.magnitude()).cross(Vec3::new(r.x, r.y, 0.0));
                f_vort += Vec2::new(cross.x, cross.y) * poly6(r.dot(r)); //glm::dot(r,r));
            }
            self.deltas[k] = delta / self.p0;
            p_i.force += f_vort;
            estimates[i].force = p_i.force; // sync the copy with the real particle
        }

        for k in 0..self.ps.len() { //(int k = 0; k < ps.size(); k++) {
            let i = self.ps[k];
            //let p_i = estimates[i]; // todo: make ref
            estimates[i].pos_guess += self.deltas[k] / (self.neighbors[k].len() + counts[i]) as f32;
        }
    }

    pub fn update_counts(&self, counts: &mut Vec<usize>) {
        // do nothing
    }

    pub fn grad(&self, estimates: &mut ParticleVec, k: usize, j: usize) -> Vec2 {
        let i = self.ps[k];
        let p_i = estimates[i]; // todo: make ref
        let p_j = estimates[j]; // todo: make ref
        let r = p_i.pos_guess - p_j.pos_guess;
        let rlen = r.magnitude(); //glm::length(r);
        if p_i != p_j {
            return -spikyGrad(&r, rlen) / (self.p0);
        }

        let mut out = Vec2::new(0.0, 0.0);
        for x in 0..self.neighbors[k].len() { //(int x = 0; x < neighbors[k].size(); x++) {
            let p_j = estimates[self.neighbors[k][x]]; // todo: make ref
            let r = p_i.pos_guess - p_j.pos_guess;
            let rlen = r.magnitude(); //glm::length(r);
            let mult = 1.0; //if p_j.phase == Phase::Solid { s_solid } else { 1.0 };
            out += mult * spikyGrad(&r, rlen);
        }

        return out / (self.p0);
    }
}



pub fn poly6(r2: f32) -> f32 {
    if r2 >= h2 {
        return 0.0;
    }
    let term2 = (h2 - r2);
    return (315.0 / (64. * PI * h9)) * (term2 * term2 * term2);
//    return (H-r) / (H*H);
}

pub fn spikyGrad(r: &Vec2, rlen2: f32) -> Vec2 {
    if (rlen2 >= h) {
        return Vec2::new(0.0, 0.0);
    }
    if (rlen2 == 0.0) {
        return Vec2::new(0.0, 0.0);
    }
    return -r.normalize() * (45.0 / (PI * h6)) * (h - rlen2) * (h - rlen2);
//    return -r / (H*H*rlen);
}
