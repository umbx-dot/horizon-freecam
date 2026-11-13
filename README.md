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
| Speed | 1 block per tick | Click to step through 0.25, 0.5, 1, 1.5, 2.5 and 4. The mouse wheel sets anything from 0.05 to 8 |
| Toggle freecam | U | The key that turns freecam on and off |

## Multiplayer

Only your camera moves. Your player sends no movement and no rotation, so the server sees you standing still. The camera can only show chunks your game has loaded. Some servers do not allow freecam, so check their rules first.

## Install

In the Horizon launcher, open Mods and install Freecam+, or use Add from GitHub with this repository's link. Then build your native mods in the launcher as usual.

## Build from source

You need Rust 1.95 or newer, Python 3.12 or newer, a Horizon checkout at the commit in `compatibility.json`, and the horizon-mixins tool at the revision listed there.

| Command | What it does |
|---|---|
| `cargo test` | Runs the tests of the camera logic |
| `python tools/bundle.py apply --horizon <horizon> --mixins-tool <horizon-mixins> --out <stage> --check-target <target>` | Applies the mod to Horizon and checks that the game compiles |
| `python tools/bundle.py package` | Writes `build/freecam_plus-<version>.hnmod` |
| `python tools/make_icon.py` | Redraws `icon.png` |

Made for Horizon Nightly 2026.11.05 (commit `66f3981`) with horizon-mixins `5a64af9`. Tested on Windows 11.

Freecam+ uses Horizon's native hook points, which Horizon keeps stable across releases: the keys (`native_key`), the camera it draws the world from (`native_camera_pose`), the player's movement input (`native_movement_input`), the mouse wheel, the tick and the Options → Mods page. Mouse look has no hook point yet, so it still hooks Horizon's `turn_player`; that is why the mod pins a Horizon commit.

## License

MIT. See [LICENSE](LICENSE).
