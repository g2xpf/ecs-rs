pub trait ResourceContainer {
    type Target;

    fn get(&self, index: usize) -> &Self::Target;
    fn get_mut(&mut self, index: usize) -> &mut Self::Target;
}
