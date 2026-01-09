#[derive(Debug, clap::Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum SubCommand {
    Daemon,
    Player(PlayerArgs)
}

#[derive(Debug, clap::Args)]
pub struct PlayerArgs {
    #[command(subcommand)]
    pub sub_command: PlayerSubCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum PlayerSubCommand {
    Play,
    Pause,
    Resume,
    Clear
}
