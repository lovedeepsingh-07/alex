# alex

> CLI daemon-based music player

```
Usage: alex [OPTIONS] <COMMAND>

Commands:
  daemon  Start the daemon by providing the path to your music folder
  status  Get information such as which song is playing, whether playback is paused or not etc
  reload  Reload the audio storage to reflect any changes to the music folder
  search  Search through the audio storage
  play    Play an audio
  next    Skip to the next song in the playing queue
  pause   Pause playback (does nothing if already paused)
  resume  Resume playback (does nothing if already resumed)
  clear   Clear playing queue
  help    Print this message or the help of the given subcommand(s)

Options:
      --port <PORT>  Server port [default: 16690]
      --just-info    Pass this flag when calling the "status" subcommands from another program
  -h, --help         Print help
  -V, --version      Print version
```

## Installation

You can either install the pre-built binary from the [github releases](https://github.com/lovedeepsingh-07/alex/releases) or you can build it yourself using [nix](https://nixos.org/). Once you installed nix.

> You have to enable flakes

```
nix build github:lovedeepsingh-07/alex
```

Or you can just use nix to install:

```
nix profile install github:lovedeepsingh-07/alex
```

Or just add it to your flake inputs:

```nix
alex.url = "github:lovedeepsingh-07/alex";
```

## Quickstart

Just go to the folder with all your music and run `alex daemon .` and daemon will be running on the port `16690`(by default) and you can use it to play music from the folder. You can change the port by passing the `--port` flag.

The great thing about alex is that you can easily integrate it into your linux environment. It has proper functionality to be used to display music info anywhere you can run a shell command, that includes status bars. It is recommended to run it on your OS startup. I do this by using this in my i3 config:

```
exec --no-startup-id alex daemon ~/Music
```

After this you can easily get the info about your music using the `alex status` sub command.

```
Usage: alex status [OPTIONS] [COMMAND]

Commands:
  current-audio   Current playing audio
  is-paused       Is the playback paused ?
  is-queue-empty  Is the playing queue empty ?
  queue           Queue
  help            Print this message or the help of the given subcommand(s)

Options:
      --port <PORT>  Server port [default: 16690]
      --just-info    Pass this flag when calling the "status" subcommands from another program
  -h, --help         Print help
```

If you are running some shell command from another program, saw "eww" then I suggest using the `--just-info` flag because it basically prints the info without pretty formatting.
