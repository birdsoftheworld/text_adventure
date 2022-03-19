use specs::prelude::*;
use specs::world::WorldExt;
use crate::command::Command;
use crate::item::NameString;
use crate::component::*;

pub mod parser;
pub mod command;
pub mod item;
pub mod english;
mod component;

pub enum ErrorType {
    CommandUnknown,
    DoesNotExist(NameString),
}

pub struct Adventure {
    world: World,
    room: Entity
}

impl Adventure {
    pub fn new() -> Adventure {
        let mut adventure = Adventure {
            world: World::new()
        };
        adventure.register_components();
        adventure
    }

    fn register_components(&mut self) {
        self.world.register::<Item>();
        self.world.register::<Container>();
        self.world.register::<Room>();
        self.world.register::<Named>();
    }
    
    pub fn execute_command(&mut self, command: Command) -> Result<String, ErrorType> {
        let res = match command {
            Command::Take(name) => {
                let found = resolve_name(&name, &self.world, self.room);
            }
            Command::TakeFrom { .. } => {}
            Command::Put { .. } => {}
            Command::Inventory => {}
            Command::Look(_) => {}
            Command::Unknown => {}
        };

        unimplemented!()
    }
}

fn resolve_name(name: &NameString, world: &World, parent: Entity) {
    let vec = resolve_name_in(name, world, parent, Vec::new());
    let name_storage = world.read_storage::<Named>();
    if let Some(named) = name_storage.get(parent) {
        if named.is_referred_to_by(name) {
            vec.push((parent, parent));
        }
    }
}

fn resolve_name_in(name: &NameString, world: &World, parent: Entity, found: Vec<(Entity, Entity)>) -> Vec<(Entity, Entity)> {
    let con_storage = world.read_storage::<Container>();
    let name_storage = world.read_storage::<Named>();
    let container = con_storage.get(parent).unwrap();

    let contents = container.get_contents();
    for entity in container {
        if let Some(named) = name_storage.get(entity) {
            if named.is_referred_to_by(name) {
                found.push((parent, entity));
            }
        }
        if con_storage.contains(entity) {
            resolve_name_in(name, world, entity, found);
        }
    }
    
    found
}