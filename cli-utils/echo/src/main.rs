use clap::Parser;
use echo::EchoCommand;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    // println!("{:o}", );
    let cmd = EchoCommand::parse();
    print!("{}", cmd.exec()?);

    Ok(())
}
