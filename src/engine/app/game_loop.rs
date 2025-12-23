use crate::engine::app::context::Context;

pub trait GameLoop: Sized {
    fn new(ctx: &mut Context) -> Self;
    fn update(&mut self, ctx: &mut Context);
    fn render(&mut self, ctx: &mut Context);
}
