pub mod parser;

mod english {
    pub const PREFIXES: [&str; 4] = ["the", "an", "a", "some"];
}

pub mod item {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    use crate::english::PREFIXES;

    #[derive(Debug, Clone)]
    pub struct ItemString(Vec<String>);
    
    impl ItemString {
        pub fn new(mut data: Vec<String>) -> ItemString {
            if data.len() > 0 && PREFIXES.contains(&&data[0][..]) {
                data.remove(0);
            }
            ItemString(data)
        }

        pub fn from_refs(data: Vec<&str>) -> ItemString {
            ItemString::new(data.iter().map(|slice| (*slice).to_owned()).collect())
        }
    }

    pub struct Entity {
        components: HashMap<TypeId, Box<dyn Component>>
    }

    impl Entity {
        fn add_component(&mut self, component: Box<dyn Component>) -> bool {
            self.components.insert(component.type_id(), component).is_some()
        }

        fn remove_component(&mut self, component: Box<dyn Component>) -> bool {
            self.components.remove(&component.type_id()).is_some()
        }

        fn get_component(&mut self, component: Box<dyn Component>) -> Option<&Box<dyn Component>> {
            self.components.get(&component.type_id())
        }
    }

    pub trait Component {

    }
}

pub mod command {
    use crate::item::ItemString;

    #[derive(Debug, Clone)]
    pub enum Command {
        Take(ItemString),
        TakeFrom {
            item: ItemString,
            source: ItemString
        },
        Put {
            item: ItemString,
            destination: ItemString
        },
        Inventory,
        Look(Option<ItemString>),
        Unknown
    }
}