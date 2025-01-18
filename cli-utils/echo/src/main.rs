use clap::Parser;
use echo::EchoArgs;

// TODO: Add backslash escaped characters support
// const ALERT_BELL: char = '\x07';
// const BACKSPACE: char = '\x08';

fn main() {
    let args = EchoArgs::parse();

    println!("{:#?}", args);
    if args.disable_new_line {
        print!("{}", args.value);
    } else {
        println!("{}", args.value);
    }
}
