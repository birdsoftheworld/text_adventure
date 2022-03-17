use crate::english::PREFIXES;

#[derive(Debug, Clone)]
pub struct NameString(Vec<String>);

impl NameString {
    pub fn new(mut data: Vec<String>) -> NameString {
        if data.len() > 0 && PREFIXES.contains(&&data[0][..]) {
            data.remove(0);
        }
        NameString(data)
    }

    pub fn from_refs(data: Vec<&str>) -> NameString {
        NameString::new(data.iter().map(|slice| (*slice).to_owned()).collect())
    }
}