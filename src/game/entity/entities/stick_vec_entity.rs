// use winit::keyboard::KeyCode;

// use crate::{game::entity::entity::{Entity, UpdateContext}, simulation::constraints::stick::StickVec};

// pub struct StickVecEntity {
//     pub stick_vec: StickVec,
// }

// impl StickVecEntity {
//     pub fn new(stick_vec: StickVec) -> Self {
//         Self {
//             stick_vec
//         }
//     }
// }

// impl Entity for StickVecEntity {
//     fn update(&mut self, context: &mut UpdateContext) {
//         self.stick_vec.execute(context.particle_vec, context.time_delta);
//     }

//     // ughly!
//     fn handle_key(&mut self, key: KeyCode, is_pressed: bool) -> bool { false }
// }