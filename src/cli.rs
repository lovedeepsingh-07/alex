#[derive(Debug, clap::Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum SubCommand {
    Daemon,
    Status,
    Search,
}

// #[derive(Debug, clap::Args)]
// pub struct StatusArgs {
//     #[command(subcommand)]
//     pub sub_command: StatusSubCommand,
// }
//
// #[derive(Debug, clap::Subcommand)]
// pub enum StatusSubCommand {
//     Daemon,
// }
