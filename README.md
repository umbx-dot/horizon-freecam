# Freecam+ for Horizon

![Freecam+ icon](icon.png)

A native mod for [Horizon](https://github.com/horizon-rust/horizon). Press a key and your camera leaves your body. Fly it anywhere and look around freely while your player stays exactly where you left it, facing the same way.

## Controls

| Input | Action |
|---|---|
| **U** | Turn freecam on or off (change it in Options, Mods, Freecam+) |
| W A S D | Move the camera |
| Space and Left Shift | Move up and down |
| Left Ctrl | Move twice as fast while held |
| Mouse | Look around. Your player does not turn |
| Mouse wheel | Change the speed. It is remembered |

Your body stays visible where you left it, and chat and menus work as usual while the camera is away.

## Settings

Open Options, Mods, Freecam+.

| Setting | Default | What it does |
|---|---|---|
| Through walls | On | When off, the camera stops at solid blocks |
| Speed | 1 block per tick | A slider from 0.05 to 8. The mouse wheel changes it while you fly |

The toggle key (U by default) is in Options, Controls.

## Multiplayer

Only your camera moves. Your player sends no movement and no rotation, so the server sees you standing still. The camera can only show chunks your game has loaded. Some servers do not allow freecam, so check their rules first.

## Install

In the Horizon launcher, open Mods and install Freecam+, or use Add from GitHub with this repository's link. Then build your native mods in the launcher as usual.

## Build from source

Freecam+ is a `.hmod`: Rust source that the launcher compiles against the prebuilt Horizon build pinned in `horizon-mod.toml` (`[native] horizon`). You need Rust and the `cargo horizon-mod` tool from Horizon.

| Command | What it does |
|---|---|
| `cargo horizon-mod check` | Fetches the pinned build's SDK into `.horizon/sdk/` and checks the mod compiles |
| `cargo test --lib` | Runs the tests of the camera logic |
| `cargo horizon-mod build` | Writes `target/horizon-mod/freecam_plus.hmod` |
| `python tools/make_icon.py` | Redraws `icon.png` |

Freecam+ uses Horizon's mod API: the toggle key mapping, `input_key`, `client_tick_post`, `movement_input`, `camera_pose`, `mouse_scrolling` and two settings the game draws in Options, Mods. Mouse look has no hook yet, so one `[[native.mixin]]` still runs at the start of Horizon's `turn_player`; that is why the mod is made for exactly one Horizon build.

## License

MIT. See [LICENSE](LICENSE).
