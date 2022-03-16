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