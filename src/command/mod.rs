use error::ChestError;

mod create;
mod add;
mod update;
mod remove;


pub trait Command {
    fn run(&self, filename: String, params: Vec<String>) -> Result<(), Box<ChestError>>;
}


pub fn command_factory(s: &str) -> Result<Box<Command>, Box<ChestError>> {
    type Err = String;

    let s = s.to_uppercase();
    match &*s {
        "CREATE" => Ok(Box::new(create::CreateCommand)),
        "ADD" => Ok(Box::new(add::AddCommand)),
        "UPDATE" => Ok(Box::new(update::UpdateCommand)),
        "REMOVE" => Ok(Box::new(remove::RemoveCommand)),
        _ => Err(Box::new(ChestError::Generic("Command not found".to_string())))
    }
}
