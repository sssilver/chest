use command::Command;
use error::ChestError;

pub struct RemoveCommand;


impl Command for RemoveCommand {
    fn run(&self, filename: String, params: Vec<String>) -> Result<(), Box<ChestError>> {
        println!("Running RemoveCommand");
        Ok(())
    }
}
