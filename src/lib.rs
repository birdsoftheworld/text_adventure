use specs::World;
use crate::command::Command;
use crate::item::ItemString;

pub mod parser;
pub mod command;
pub mod item;
mod english;

pub enum ErrorType {
    CommandUnknown,
    DoesNotExist(ItemString),
}

pub struct Adventure {
    world: World
}

impl Adventure {
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