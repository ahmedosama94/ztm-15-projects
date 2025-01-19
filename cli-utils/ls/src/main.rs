use clap::Parser;
use ls::LsCommand;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let cmd = LsCommand::parse();

    let output = cmd.exec()?;
    if let Some(total_blocks) = output.total_blocks() {
        println!("total {}", total_blocks);
    }

    for (idx, entry) in output.entries().iter().enumerate() {
        if let Some(metadata) = entry.metadata() {
            print!("{} ", metadata);
        }

        print!("{}", entry.file_or_dir_name());

        if idx < output.entries().len() - 1 {
            print!("{}", output.separator());
        }
    }
    println!();

    Ok(())
}
