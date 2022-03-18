use specs::prelude::*;
use specs::world::WorldExt;
use crate::command::Command;
use crate::item::NameString;

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
    world: World
}

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