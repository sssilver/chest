use std::fs::File;

use command;
use error::ChestError;


pub struct CreateCommand;

impl command::Command for CreateCommand {
    fn run(&self, filename: String, params: Vec<String>) -> Result<(), Box<ChestError>> {
        if params.len() != 0 {
            return Err(Box::new(ChestError::Generic("Params length error".to_string())));
        }

        print!("Creating chest {}...", filename);

        match File::create(format!("{}.chest", filename)) {
            Ok(_) => {},
            Err(err) => return Err(Box::new(ChestError::Generic("Error with the filesystem".to_string())))
        }

        println!(" OK");

        Ok(())
    }
}
