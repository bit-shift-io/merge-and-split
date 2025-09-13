use crate::platform::{app::App, plugin::Plugin};


pub struct BasicParticles {

}

impl BasicParticles {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for BasicParticles {
    fn init(&self, app: &mut App) {
        println!("basic particles init");
    }

    fn update(&self, app: &mut App) {

    }

    fn render(&self, app: &mut App) {
        
    }
}