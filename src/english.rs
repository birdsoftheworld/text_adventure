pub const PREFIXES: [&str; 4] = ["the", "an", "a", "some"];

pub mod error {
    use crate::parser::command::error::{CommandParseError, CommandParseType};

    pub fn interpret_cmd_err(err: CommandParseError<&str>) -> String {
        match err {
            CommandParseError::MissingArg(t, i) => {
                match t {
                    CommandParseType::Take => "Take what?",
                    CommandParseType::TakeFrom => "Take from where?",
                    CommandParseType::Put => match i {
                        0 => "Put what, and where?",
                        1 => "Put it where?",
                        _ => unreachable!()
                    },
                    CommandParseType::Look => "Look at what?",
                    _ => unimplemented!()
                }
            },
            CommandParseError::Unknown => "I don't know that word.",
            _ => unimplemented!()
        }.to_owned()
    }
}