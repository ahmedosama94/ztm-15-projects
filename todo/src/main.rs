use clap::Parser;
use color_eyre::Result;
use todo::TodoCli;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    color_eyre::install()?;

    let cmd = TodoCli::parse();
    let output = cmd.exec().await?;
    println!("{}", output);

    Ok(())
}
