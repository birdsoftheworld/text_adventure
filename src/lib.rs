pub mod parser;

pub mod item {
    #[derive(Debug, Clone)]
    pub struct ItemString(Vec<String>);

    const PREFIXES: [&str; 4] = ["the", "an", "a", "some"];
    
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