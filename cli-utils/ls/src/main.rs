use clap::Parser;
use color_eyre::eyre::Result;
use ls::LsCommand;

fn main() -> Result<()> {

    let cmd = LsCommand::parse();

    let output = cmd.exec()?;
    if let Some(total_blocks_str) = output.total_blocks_str() {
        println!("{}", total_blocks_str);
    }

    let max_str_length = if cmd.human_readable() {
        4
    } else {
        format!("{}", output.max_size()).len()
    };

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
