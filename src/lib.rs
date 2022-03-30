use specs::prelude::*;
use specs::world::WorldExt;
use crate::command::Command;
use crate::item::NameString;
use crate::component::*;
use crate::story::create_story;

pub mod parser;
pub mod command;
pub mod item;
pub mod english;
mod component;

mod story;

pub enum ErrorType {
    CommandUnknown,
    DoesNotExist(NameString),
}

struct State {
    room: Entity
}

impl State {
    fn new(room: Entity) -> State {
        State {
            room
        }
    }
}

pub struct Adventure {
    world: World,
    state: State
}

impl Adventure {
    pub fn new() -> Adventure {
        let mut world = World::new();

        create_story(&mut world);

        let current_room_storage = world.read_storage::<CurrentRoom>();
        for p in current_room_storage {

        }

        let mut adventure = Adventure {
            world,
            state: State::new(current_room)
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
    let mut vec = Vec::new();
    resolve_name_in(name, world, parent, &mut vec);
    let name_storage = world.read_storage::<Named>();
    if let Some(named) = name_storage.get(parent) {
        if named.is_referred_to_by(name) {
            vec.push((parent, parent));
        }
    }
}

fn resolve_name_in(name: &NameString, world: &World, parent: Entity, mut found: &Vec<(Entity, Entity)>) {
    let con_storage = world.read_storage::<Container>();
    let name_storage = world.read_storage::<Named>();
    let container = con_storage.get(parent).unwrap();

    let contents = container.get_contents();
    for entity in contents {
        if let Some(named) = name_storage.get(*entity) {
            if named.is_referred_to_by(name) {
                found.push((parent, *entity));
            }
        }
        if con_storage.contains(*entity) {
            resolve_name_in(name, world, *entity, found);
        }
    }
}