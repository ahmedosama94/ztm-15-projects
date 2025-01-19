use clap::Parser;
use echo::EchoCommand;

fn main() {
    let args = EchoCommand::parse();
    let output = args.exec();

    print!("{}", output);
}
