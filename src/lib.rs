pub mod parser;

mod english {
    pub const PREFIXES: [&str; 4] = ["the", "an", "a", "some"];
}

pub mod item {
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