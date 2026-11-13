use super::perspective::CameraType;
use super::umbx_freecam as freecam;
use super::Game;
use std::ops::ControlFlow;

const MOVES: [&str; 7] = [
    "key.keyboard.w",
    "key.keyboard.s",
    "key.keyboard.a",
    "key.keyboard.d",
    "key.keyboard.space",
    "key.keyboard.left.shift",
    "key.keyboard.left.control",
];
const VIEWS: [CameraType; 3] = [CameraType::FirstPerson, CameraType::ThirdPersonBack, CameraType::ThirdPersonFront];

fn settings(game: &Game<'_>) -> freecam::Settings {
    freecam::settings(|key| game.native_setting(freecam::ID, key))
}

fn store(game: &mut Game<'_>, settings: freecam::Settings) {
    freecam::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(freecam::ID, key, &value);
    }
}

pub(super) fn key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if freecam::active()
        && let Some(input) = MOVES.iter().position(|m| *m == key)
        && (!pressed || game.native_playing())
    {
        freecam::hold(input, pressed);
    }
    if !pressed || !game.native_on(freecam::ID) || !game.native_playing() || key != game.native_mod_key(freecam::ID, freecam::KEY, freecam::DEFAULT_KEY) {
        return;
    }
    if freecam::active() {
        exit(game);
    } else {
        enter(game);
    }
}

fn enter(game: &mut Game<'_>) {
    let Some(player) = game.player.as_ref() else { return };
    let eye = [player.position.x, player.position.y + f64::from(game.eye), player.position.z];
    settings(game);
    let view = VIEWS.iter().position(|v| *v == game.camera_type).unwrap_or(0) as u8;
    freecam::start(eye, game.yaw, game.pitch, view);
    game.camera_type = CameraType::ThirdPersonBack;
}

fn exit(game: &mut Game<'_>) {
    let Some(view) = freecam::stop() else { return };
    game.camera_type = VIEWS[usize::from(view).min(VIEWS.len() - 1)];
}

pub(super) fn movement(forward: &mut f32, strafe: &mut f32, jump: &mut bool, sneak: &mut bool) {
    if freecam::active() {
        *forward = 0.0;
        *strafe = 0.0;
        *jump = false;
        *sneak = false;
    }
}

pub(super) fn turn(xo: f64, yo: f64) -> ControlFlow<()> {
    if freecam::turn(xo, yo) { ControlFlow::Break(()) } else { ControlFlow::Continue(()) }
}

pub(super) fn camera(partial: f32, position: &mut [f64; 3], yaw: &mut f32, pitch: &mut f32) {
    if let Some((at, y, p)) = freecam::view(partial) {
        *position = at;
        *yaw = y;
        *pitch = p;
    }
}

pub(super) fn tick(game: &mut Game<'_>) {
    if !freecam::active() {
        return;
    }
    if game.player.is_none() || !game.native_on(freecam::ID) {
        exit(game);
        return;
    }
    game.camera_type = CameraType::ThirdPersonBack;
    let noclip = settings(game).noclip;
    let game = &*game;
    freecam::tick(|[x, y, z]| !noclip && game.res.facts().get(game.world.block(x, y, z)).full_collision);
}

pub(super) fn wheel(game: &mut Game<'_>, steps: f32, claimed: &mut bool) {
    if !game.native_on(freecam::ID) {
        return;
    }
    if let Some(settings) = freecam::wheel(steps) {
        store(game, settings);
        *claimed = true;
    }
}

pub(super) fn listed(mods: &mut Vec<super::native_hooks::NativeMod>) {
    mods.push(super::native_hooks::NativeMod {
        id: freecam::ID.into(),
        title: format!("Freecam+ {}", freecam::VERSION),
        description: freecam::DESCRIPTION.into(),
        keys: vec![[freecam::KEY.into(), "Toggle freecam".into(), freecam::DEFAULT_KEY.into()]],
    });
}

pub(super) fn page(game: &Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id == freecam::ID {
        buttons.extend(settings(game).labels());
    }
}

pub(super) fn click(game: &mut Game<'_>, mod_id: &str, button: usize) {
    if mod_id != freecam::ID {
        return;
    }
    let mut settings = settings(game);
    settings.click(button);
    store(game, settings);
}
