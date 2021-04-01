use std::{
    convert::{TryFrom, TryInto},
    ops::{Index, IndexMut},
};

#[derive(Default)]
pub struct World<Component> {
    data: Vec<Component>,
}

pub struct Entity(u64);

impl<C> Index<Entity> for World<C> {
    type Output = C;

    fn index(&self, index: Entity) -> &Self::Output {
        &self.data[usize::try_from(index.0).unwrap()]
    }
}

impl<C> IndexMut<Entity> for World<C> {
    fn index_mut(&mut self, index: Entity) -> &mut Self::Output {
        &mut self.data[usize::try_from(index.0).unwrap()]
    }
}

impl<C> World<C> {
    pub fn add(&mut self, value: C) -> Entity {
        let idx = self.data.len();
        self.data.push(value);
        Entity(idx.try_into().unwrap())
    }

    // TODO: Supporting removing data from the world
}

#[derive(Default)]
pub struct Schedule<Component> {
    systems: Vec<Box<dyn System<Component>>>,
}

impl<W> Schedule<W> {
    pub fn with_system<T: System<W>>(mut self, it: T) -> Self {
        self.add_system(it);
        self
    }
    pub fn add_system<T: System<W>>(&mut self, it: T) {
        self.systems.push(Box::new(it))
    }
}

pub trait System<W>: 'static {
    fn run(&mut self, world: &mut World<W>);
}

impl<T, W> System<W> for T
where
    T: FnMut(&mut World<W>) + 'static,
{
    fn run(&mut self, world: &mut World<W>) {
        self(world)
    }
}
