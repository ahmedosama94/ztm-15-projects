use clap::Parser;
use echo::EchoCommand;

fn main() {
    let cmd = EchoCommand::parse();

    print!("{}", cmd.exec());
}
