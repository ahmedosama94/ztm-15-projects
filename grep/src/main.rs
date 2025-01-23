use clap::Parser;
use color_eyre::Result;
use grep::GrepCommand;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cmd = GrepCommand::parse();
    println!("{}", cmd.exec()?);

    Ok(())
}
