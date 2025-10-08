use super::level_builder::LevelBuilderContext;


pub trait LevelBuilderOperation {
    fn type_name(&self) -> &str;

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync>;

    fn default_spawn_chance(&self) -> f32 {
        1.0
    }

    fn prepare(&self, level_builder_context: &mut LevelBuilderContext, level_builder_operations: &mut Vec<(f32, Box<dyn LevelBuilderOperation + Send + Sync>)>) {
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext);
}