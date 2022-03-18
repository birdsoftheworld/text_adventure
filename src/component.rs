use std::collections::HashSet;
use specs::world::Index;
use specs::Component;
use specs::prelude::*;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Item;

#[derive(Component, Default)]
pub struct Container {
    contents: HashSet<Index>
}

#[derive(Component)]
pub struct Room {
    connections: Vec<Index>
}

#[derive(Component, Default)]
pub struct Named(String);