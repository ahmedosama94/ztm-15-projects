use clap::Parser;
use color_eyre::Result;
use todo::Todo;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cmd = Todo::parse();
    println!("{}", cmd.exec()?);

    Ok(())
}
