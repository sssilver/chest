mod error;
mod command;
mod args;


fn main() {
    let args = match args::Args::from_vector(std::env::args().collect()) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return
        }
    };

    // Run the command
    match args.command.run(args.filename, args.params) {
        Ok(_) => {},
        Err(err) =>  {
            println!("Error running the command:\n    {}", err);
            return
        }
    }
}
