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