use clap::Parser;
use ls::LsCommand;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let cmd = LsCommand::parse();

    let output = cmd.exec()?;
    if let Some(total_blocks) = output.total_blocks() {
        println!("total {}", total_blocks);
    }

    let max_str_length = format!("{}", output.max_size()).len();

    for (idx, entry) in output.entries().iter().enumerate() {
        if let Some(extra) = entry.extra() {
            print!(
                "{} {} {} {} {} {} ",
                extra.metadata(),
                extra.links(),
                extra.user().unwrap().name().to_str().unwrap_or("N/A"),
                extra.group().unwrap().name().to_str().unwrap_or("N/A"),
                extra.size_str(max_str_length),
                extra.modified_str(),
            );
        }

        print!("{}", entry.file_or_dir_name());

        if idx < output.entries().len() - 1 {
            print!("{}", output.separator());
        }
    }
    println!();

    Ok(())
}
