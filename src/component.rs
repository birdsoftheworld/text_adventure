use std::collections::HashSet;
use specs::world::Entity;
use specs::Component;
use specs::prelude::*;
use crate::item::NameString;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Item;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Visible;

#[derive(Component, Default)]
pub struct Container {
    contents: HashSet<Entity>
}

impl Container {
    pub fn new() -> Container {
        Container {
            contents: HashSet::new()
        }
    }

    pub fn get_contents(&self) -> &HashSet<Entity> {
        &self.contents
    }

    pub fn add(&mut self, entity: Entity) {
        self.contents.insert(entity);
    }

    pub fn remove(&mut self, entity: Entity) {
        self.contents.remove(&entity);
    }
}

#[derive(Component)]
pub struct Room {
    connections: HashSet<Entity>
}

impl Room {
    pub fn new() -> Room {
        Room {
            connections: HashSet::new()
        }
    }

    pub fn get_connections(&self) -> &HashSet<Entity> {
        &self.connections
    }

    pub fn add_connection(&mut self, connection: Entity) {
        self.connections.insert(connection);
    }

    pub fn remove_connection(&mut self, connection: Entity) {
        self.connections.remove(&connection);
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct CurrentRoom;

#[derive(Component, Default)]
pub struct Named {
    canonical: NameString,
    names: Vec<NameString>
}

#[derive(Component, Default)]
pub struct Description {
    description: String
}

impl Description {
    pub fn new() -> Description {
        Description {
            description: String::new()
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description[..]
    }

    pub fn set_description(&mut self, string: &str) {
        self.description = string.to_owned();
    }
}

impl Named {
    pub fn is_referred_to_by(&self, other: &NameString) -> bool {
        self.names.iter().any(|s| s == other)
    }
}