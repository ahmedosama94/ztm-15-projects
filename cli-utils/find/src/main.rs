use clap::Parser;
use color_eyre::Result;
use find::FindCommand;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cmd = FindCommand::parse();
    let result = cmd.exec()?;

    println!("{}", result);

    Ok(())
}
