use command;
use error::ChestError;
use std::error::Error;


pub struct Args {
    pub filename: String,
    pub command: Box<command::Command>,
    pub params: Vec<String>
}

impl Args {
    pub fn from_vector(raw_args: Vec<String>) -> Result<Args, Box<ChestError>> {
        if raw_args.len() < 3 {
            return Err(Box::new(ChestError::Generic("Number of arguments too low".to_string())));
        }

        let filename = &*raw_args[1];

        let command = &*raw_args[2];
        let command = match command::command_factory(command) {
            Ok(command) => command,
            Err(err) => { return Err(err); }
        };

        Ok(Args {
            filename: filename.to_string(),
            command: command,
            params: raw_args[3 .. raw_args.len()].to_vec()  // 0: chest, 1: command, 2: chest name
        })
    }
}

