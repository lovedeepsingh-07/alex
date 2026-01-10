#[derive(Debug, clap::Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Debug, clap::Subcommand, PartialEq)]
pub enum SubCommand {
    Daemon,
    Reload,
    Search,
    Player {
        #[command(subcommand)]
        sub_command: PlayerSubCommand,
    },
}

#[derive(Debug, clap::Subcommand, PartialEq)]
pub enum PlayerSubCommand {
    Play { audio_label: String },
    Pause,
    Resume,
    Clear,
}
