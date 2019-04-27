use super::Resource;

#[derive(Debug, Clone)]
pub struct ResourceList<R: Resource>(pub R);

impl ResourceList<()> {
    pub fn new() -> Self {
        ResourceList(())
    }
}

impl<S> ResourceList<S>
where
    S: Resource,
{
    pub fn push<T>(self, t: T) -> ResourceList<(T, Self)>
    where
        T: Resource,
    {
        ResourceList((t, self))
    }
}
