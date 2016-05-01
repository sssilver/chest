use command::Command;
use error::ChestError;

pub struct UpdateCommand;


impl Command for UpdateCommand {
    fn run(&self, filename: String, params: Vec<String>) -> Result<(), Box<ChestError>> {
        println!("Running UpdateCommand");
        Ok(())
    }
}
