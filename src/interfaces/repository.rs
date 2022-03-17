pub trait Repository {
    fn add(&mut self, title: String) -> bool;
    fn list(&mut self);
    fn delete(&mut self);
}
