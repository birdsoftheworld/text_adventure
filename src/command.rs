use crate::item::NameString;

#[derive(Debug, Clone)]
pub enum Command {
    Take(NameString),
    TakeFrom {
        item: NameString,
        source: NameString
    },
    Put {
        item: NameString,
        destination: NameString
    },
    Inventory,
    Look(Option<NameString>),
    Unknown
}