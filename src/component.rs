use std::collections::HashSet;
use specs::world::Entity;
use specs::Component;
use specs::prelude::*;
use crate::item::NameString;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Item;

#[derive(Component, Default)]
pub struct Container {
    contents: HashSet<Entity>
}

impl Container {
    pub fn get_contents(&self) -> HashSet<Entity> {
        
    }
}

#[derive(Component)]
pub struct Room {
    connections: Vec<Entity>
}

#[derive(Component, Default)]
pub struct Named {
    canonical: NameString,
    names: Vec<NameString>
}

impl Named {
    pub fn is_referred_to_by(&self, other: &NameString) -> bool {
        self.names.iter().any(|s| s == other)
    }
}