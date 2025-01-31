use cat::CatCommand;
use clap::Parser;

fn main() {
    let command = CatCommand::parse();
    match command.exec() {
        Ok(output ) => print!("{}", output),
        Err(err) => {
            panic!("{}", err);
        }
    };
}
