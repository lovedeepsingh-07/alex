mod cli;

use clap::Parser;

fn main() {
    let cli_args = cli::CliArgs::parse();
    match cli_args.sub_command {
        cli::SubCommand::Daemon => {
            println!("start the daemon");
        },
        cli::SubCommand::Status(status_args) => {
            match status_args.sub_command {
                cli::StatusSubCommand::Daemon => {
                    println!("give the status of the daemon");
                }
            }
        }
        cli::SubCommand::Search => {
            println!("search for a song");
        }
    }
}
