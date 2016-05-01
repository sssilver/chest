use command::Command;
use error::ChestError;

pub struct AddCommand;


impl Command for AddCommand {
    fn run(&self, filename: String, params: Vec<String>) -> Result<(), Box<ChestError>> {
        println!("Running AddCommand");
        Ok(())
    }
}
