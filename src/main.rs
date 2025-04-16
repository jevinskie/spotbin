use clap::Parser;

use commands::{MainCommand, SubCommand};

mod cli_routine;
mod commands;

fn main() {
    let args = MainCommand::parse();

    match args.subcmd {
        SubCommand::Dump(opts) => {
            cli_routine::dump(opts);
        }
    }
}
