pub const PREFIXES: [&str; 4] = ["the", "an", "a", "some"];

pub mod error {
    use crate::parser::command::error::CommandParseError;
    
    pub fn interpret_cmd_err(err: CommandParseError<&str>) -> String {
        match err {
            MissingArg(t, i) => {
                match t {
                    Take => "Take what?",
                    TakeFrom => "Take from where?",
                    Put => match i {
                        0 => "Put what, and where?",
                        1 => "Put it where?",
                        _ => unreachable!()
                    },
                    Look => "Look at what?",
                    _ => unimplemented!()
                }
            },
            Unknown => "I don't know that word.",
            _ => unimplemented!()
        }.to_owned()
    }
}