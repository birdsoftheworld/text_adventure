use my_project::parser::command::parse_full_command;
use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let com = buffer.to_lowercase();
        let parsed = parse_full_command(&com).unwrap();
        println!("{:?}", parsed);
    }
}