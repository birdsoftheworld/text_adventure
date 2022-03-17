use specs::prelude::*;
use specs::Component;
use specs::world::{Index, WorldExt};
use crate::command::Command;
use crate::item::NameString;

pub mod parser;
pub mod command;
pub mod item;
mod english;

pub enum ErrorType {
    CommandUnknown,
    DoesNotExist(NameString),
}

pub struct Adventure {
    world: World
}

#[derive(Component)]
#[storage(NullStorage)]
pub struct Item;

#[derive(Component, Default)]
pub struct Container {
    contents: Vec<Index>
}

#[derive(Component)]
pub struct Room {
}

#[derive(Component, Default)]
pub struct Named(String);

impl Adventure {
    pub fn new() -> Adventure {
        Adventure {
            world: World::new()
        }
    }
    
    pub fn execute_command(&mut self, command: Command) -> Result<String, ErrorType> {
        let res = match command {
            Command::Take(_) => {}
            Command::TakeFrom { .. } => {}
            Command::Put { .. } => {}
            Command::Inventory => {}
            Command::Look(_) => {}
            Command::Unknown => {}
        };

        unimplemented!()
    }
}