use clap::Parser;
use echo::EchoArgs;

fn main() {
    let args = EchoArgs::parse();

    args.exec();
}
