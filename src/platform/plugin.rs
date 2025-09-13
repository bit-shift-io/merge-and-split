pub struct Plugin {

}

pub trait Plugin {
    fn init(&self, app: &mut App);
    //fn event(&self, app: &mut App);
    fn update(&self, app: &mut App);
    fn render(&self, app: &mut App);
}
