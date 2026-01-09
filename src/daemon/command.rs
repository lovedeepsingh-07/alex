use crate::daemon::player;

#[derive(Debug)]
pub enum Command {
    PlayerPlay,
    PlayerPause,
    PlayerResume,
    PlayerClear
}

pub fn handle_command(cmd: Command, player: &mut player::Player) {
    log::info!("{:#?}", cmd);
    // player.play(player.music_files[0].clone();
    // match cmd {
    //     Command::PlayerPlay => {},
    //     Command::PlayerPause => {},
    //     Command::PlayerResume => {},
    //     Command::PlayerClear => {},
    // }
}
