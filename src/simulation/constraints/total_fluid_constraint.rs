use std::collections::HashMap;
use std::f32::consts::PI;

use crate::{core::math::vec2::Vec2, simulation::particles::{particle::Phase, particle_vec::ParticleVec}};

const H: f32 = 2.0;
const H2: f32 = 4.0;
const H6: f32 = 64.0;
const H9: f32 = 512.0;

const S_SOLID: f32 = 0.0; // Fluid-solid coupling constant
const RELAXATION: f32 = 0.01; // Epsilon in gamma correction denominator

// Pressure terms
const K_P: f32 = 0.1;
const E_P: f32 = 4.0;
const DQ_P: f32 = 0.2;


pub struct TotalFluidConstraint {
    pub p0: f32,
    pub neighbors: Vec<Vec<usize>>,
    pub ps: Vec<usize>,
    pub deltas: Vec<Vec2>,
    pub lambdas: HashMap<usize, f32>,
}

fn init_neighbours_and_deltas_for_size(size: usize) -> (Vec<Vec<usize>>, Vec<Vec2>) {
    let mut neighbors = Vec::<Vec<usize>>::new(); //[particles->size()];
    let mut deltas = Vec::<Vec2>::new(); //new glm::dvec2[particles->size()];

    for _ in 0..size { //for (int i = 0; i < particles->size(); i++) {
        neighbors.push(Vec::<usize>::new());
        deltas.push(Vec2::new(0.0, 0.0));
    }

    (neighbors, deltas)
}

impl TotalFluidConstraint {
    pub fn new(density: f32, particles: &Vec<usize>) -> Self {
        // let mut neighbors = Vec::<Vec<usize>>::new(); //[particles->size()];
        // let mut deltas = Vec::<Vec2>::new(); //new glm::dvec2[particles->size()];

        let num_particles = particles.len();

        let mut ps= Vec::<usize>::new();
        for i in 0..num_particles { //for (int i = 0; i < particles->size(); i++) {
            ps.push(particles[i]);
            // neighbors.push(Vec::<usize>::new());
            // deltas.push(Vec2::new(0.0, 0.0));
        }

        let (neighbors, deltas) = init_neighbours_and_deltas_for_size(num_particles);

        Self {
            p0: density,
            neighbors,
            ps,
            deltas,
            lambdas: HashMap::new(),
        }
    }

    pub fn project(&mut self, estimates: &mut ParticleVec, counts: &Vec<usize>) {
        // Find neighboring particles and estimate pi for each particle
        self.lambdas.clear();
        for k in 0..self.ps.len() { //for (int k = 0; k < ps.size(); k++) {
            self.neighbors[k].clear();
            let i = self.ps[k];
            let p_i = estimates[i]; // todo: make ref?
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
                    if rlen2 < H2 {

                        // Found a neighbor! Remember it and add to pi and the gamma denominator
                        self.neighbors[k].push(j);
                        let mut incr = poly6(rlen2) / p_j.imass;
                        if p_j.phase == Phase::Solid {
                            incr *= S_SOLID;
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
            let lambda = -((pi / self.p0) - 1.) / (denom + RELAXATION);
            self.lambdas.insert(i, lambda); //self.lambdas[i] = lambda;
        }

        // Compute actual deltas
        for k in 0..self.ps.len() { //(int k = 0; k < ps.size(); k++) {
            let mut delta = Vec2::new(0.0, 0.0);
            let i = self.ps[k];
            let p_i = estimates[i]; // todo: make ref?

            for x in 0..self.neighbors[k].len() { // (int x = 0; x < neighbors[k].size(); x++) {
                let j = self.neighbors[k][x];
                if i == j {
                    continue;
                }
                let p_j = estimates[j]; // todo: make ref
                let r = p_i.pos_guess - p_j.pos_guess;
                let rlen = r.magnitude(); //glm::length(r);
                let sg = spiky_grad(&r, rlen);
                let lambda_corr = -K_P * (poly6(rlen * rlen) / poly6(DQ_P * DQ_P * H * H)).powf(E_P); //pow(, E_P);

                let lambdas_i = match self.lambdas.get(&i) {
                    Some(value) => *value,
                    None => 0.0
                };
                let lambdas_j = match self.lambdas.get(&j) {
                    Some(value) => *value,
                    None => 0.0
                };

                delta += (lambdas_i + lambdas_j + lambda_corr) * sg;
            }
            self.deltas[k] = delta / self.p0;
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
            return -spiky_grad(&r, rlen) / (self.p0);
        }

        let mut out = Vec2::new(0.0, 0.0);
        for x in 0..self.neighbors[k].len() { //(int x = 0; x < neighbors[k].size(); x++) {
            let p_j = estimates[self.neighbors[k][x]]; // todo: make ref
            let r = p_i.pos_guess - p_j.pos_guess;
            let rlen = r.magnitude(); //glm::length(r);
            let mult = if p_j.phase == Phase::Solid { S_SOLID } else { 1.0 };
            out += mult * spiky_grad(&r, rlen);
        }

        return out / (self.p0);
    }


    pub fn add_particle(&mut self, index: usize) {
        // self.neighbors.clear(); //delete[] neighbors;
        // self.deltas.clear(); //delete[] deltas;
        // //numParticles++;
        // neighbors = new QList<int>[numParticles];
        // deltas = new glm::dvec2[numParticles];
        self.ps.push(index);

        let (neighbors, deltas) = init_neighbours_and_deltas_for_size(self.ps.len());
        self.neighbors = neighbors;
        self.deltas = deltas;
    }

    pub fn removE_Particle(&mut self, index: usize) {
    //     self.neighbors.clear(); //delete[] neighbors;
    //     self.deltas.clear(); //delete[] deltas;
    // //    if(ps.contains(index)) {
    //         //numParticles--;
    //         neighbors = new QList<int>[numParticles];
    //         deltas = new glm::dvec2[numParticles];
            self.ps.remove(index);

            let (neighbors, deltas) = init_neighbours_and_deltas_for_size(self.ps.len());
            self.neighbors = neighbors;
            self.deltas = deltas;
    //    }
    }
}

pub struct TotalFluidConstraintVec(pub Vec<TotalFluidConstraint>);

impl TotalFluidConstraintVec {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn update_counts(&self, counts: &mut Vec<usize>) {
        for c in &self.0 {
            c.update_counts(counts);
        }
    }

    pub fn solve(&mut self, particles: &mut ParticleVec, counts: &Vec<usize>) {
        for c in &mut self.0 {
            c.project(particles, counts);
        }
    }

    pub fn push(&mut self, c: TotalFluidConstraint) {
        self.0.push(c);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, TotalFluidConstraint> {
        self.0.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl std::ops::Index<usize> for TotalFluidConstraintVec {
    type Output = TotalFluidConstraint;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for TotalFluidConstraintVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}



pub fn poly6(r2: f32) -> f32 {
    if r2 >= H2 {
        return 0.0;
    }
    let term2 = H2 - r2;
    return (315.0 / (64. * PI * H9)) * (term2 * term2 * term2);
//    return (H-r) / (H*H);
}

pub fn spiky_grad(r: &Vec2, rlen2: f32) -> Vec2 {
    if rlen2 >= H {
        return Vec2::new(0.0, 0.0);
    }
    if rlen2 == 0.0 {
        return Vec2::new(0.0, 0.0);
    }
    return -r.normalize() * (45.0 / (PI * H6)) * (H - rlen2) * (H - rlen2);
//    return -r / (H*H*rlen);
}
