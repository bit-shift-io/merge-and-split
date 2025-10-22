use cgmath::InnerSpace;

use crate::{math::vec2::Vec2, particles::{body::Body, particle::Particle, particle_vec::ParticleVec}};


pub struct RigidContactConstraint {
   i1: usize,
   i2: usize, 

   n: Vec2,
   d: f32,
   stable: bool,
} 

impl RigidContactConstraint {
    pub fn new(i1: usize, i2: usize, stable: bool) -> Self {
        Self {
            i1,
            i2,
            stable,
            n: Vec2::new(0.0, 0.0),
            d: 0.0
        }
    }

    pub fn project(&mut self, estimates: &mut ParticleVec, counts: &Vec<usize>, bodies: &Vec<Body>) {
        let mut p1 = estimates[self.i1]; // todo: use ref's, but has safety issues
        let mut p2 = estimates[self.i2];
        let dat1 = p1.get_sdf_data(bodies, self.i1);
        let dat2 = p2.get_sdf_data(bodies, self.i2);

        if dat1.distance < 0.0 || dat2.distance < 0.0 {
            let x12 = p2.get_p(self.stable) - p1.get_p(self.stable);
            let len = x12.magnitude();
            let d = (p1.radius + p2.radius) - len;
            if d < f32::EPSILON {
                return;
            }
            self.n = x12 / len;
        } else {
            if dat1.distance < dat2.distance {
                self.d = dat1.distance;
                self.n = dat1.gradient;
            } else {
                self.d = dat2.distance;
                self.n = -dat2.gradient;
            }

            if self.d < (p1.radius + p2.radius) + f32::EPSILON {
                if self.init_boundary(&p1, &p2) {
                    return;
                }
            }
        }

        let w_sum = p1.tmass + p2.tmass;
        let dp = (1.0 / w_sum) * self.d * self.n;
        let dp1 = -p1.tmass * dp  / counts[self.i1] as f32;
        let dp2 = p2.tmass * dp / counts[self.i2] as f32;

        if !self.stable {
            p1.pos_guess += dp1;
            p2.pos_guess += dp2;

            estimates[self.i1].pos_guess = p1.pos_guess; // copy changes to copies back into estimates (hack to work around unsafe for now)
            estimates[self.i2].pos_guess = p2.pos_guess;
        } else {
            p1.pos += dp1;
            p2.pos += dp2;

            estimates[self.i1].pos = p1.pos; // copy changes to copies back into estimates (hack to work around unsafe for now)
            estimates[self.i2].pos = p2.pos;
        }


        // Apply friction
        let nf = self.n.normalize();
        let dpf = (p1.pos_guess - p1.pos) - (p2.pos_guess - p2.pos);
        let dpt = dpf - dpf.dot(nf) * nf;
        let ldpt = dpt.magnitude();
        if ldpt < f32::EPSILON {
            return;
        }
        let s_fric = (p1.s_friction * p2.s_friction).sqrt();
        let k_fric = (p1.k_friction * p2.k_friction).sqrt();

        if ldpt < s_fric * self.d {
            if self.stable {
                p1.pos -= dpt * p1.tmass / w_sum;
                p2.pos += dpt * p2.tmass / w_sum;

                estimates[self.i1].pos = p1.pos; // copy changes to copies back into estimates (hack to work around unsafe for now)
                estimates[self.i2].pos = p2.pos;
            }
            p1.pos_guess -= dpt * p1.tmass / w_sum;
            p2.pos_guess += dpt * p2.tmass / w_sum;

            estimates[self.i1].pos_guess = p1.pos_guess; // copy changes to copies back into estimates (hack to work around unsafe for now)
            estimates[self.i2].pos_guess = p2.pos_guess;
        } else {
            let delta = dpt * f32::min(k_fric * self.d / ldpt, 1.);
            if self.stable {
                p1.pos -= delta * p1.tmass / w_sum;
                p2.pos += delta * p2.tmass / w_sum;

                estimates[self.i1].pos = p1.pos; // copy changes to copies back into estimates (hack to work around unsafe for now)
                estimates[self.i2].pos = p2.pos;
            }
            p1.pos_guess -= delta * p1.tmass / w_sum;
            p2.pos_guess += delta * p2.tmass / w_sum;

            estimates[self.i1].pos_guess = p1.pos_guess; // copy changes to copies back into estimates (hack to work around unsafe for now)
            estimates[self.i2].pos_guess = p2.pos_guess;
        }
    }

    pub fn update_counts(&self, counts: &mut Vec<usize>) {
        counts[self.i1] += 1;
        counts[self.i2] += 1;
    }


    pub fn init_boundary(&mut self, p1: &Particle, p2: &Particle) -> bool {
        let mut x12 = p1.get_p(self.stable) - p2.get_p(self.stable);
        let len = x12.magnitude();
        self.d = (p1.radius + p2.radius) - len;
        if self.d < f32::EPSILON {
            return true;
        }

        if len > f32::EPSILON {
            x12 =  x12 / len;
        } else {
            x12 = Vec2::new(0.0, 1.0);
        }

        let dp = x12.dot(self.n);
        if dp < 0.0 {
            self.n = x12 - 2.0 * dp * self.n;
        } else {
            self.n = x12;
        }
        return false;
    }
}