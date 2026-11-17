mod runtime;

use mod_api::{
    self, CameraPoseEvent, CameraType, ClientHost, ClientTickPostEvent, Flow, Hooks, InputKeyEvent,
    KeyAction, KeyRegistration, MouseScrollingEvent, MovementInputEvent, Registrar,
    RegistrationContext, SettingSpec,
};
use runtime as freecam;

pub use runtime::turn;

const ID: &str = "freecam_plus";
const TOGGLE: &str = "toggle";
const MOVES: [&str; 7] = [
    "key.keyboard.w",
    "key.keyboard.s",
    "key.keyboard.a",
    "key.keyboard.d",
    "key.keyboard.space",
    "key.keyboard.left.shift",
    "key.keyboard.left.control",
];
const VIEWS: [CameraType; 3] = [
    CameraType::FirstPerson,
    CameraType::ThirdPersonBack,
    CameraType::ThirdPersonFront,
];

#[derive(Default)]
struct Mod;

fn settings(host: &dyn ClientHost) -> freecam::Settings {
    freecam::Settings::new(
        host.setting_bool("noclip").unwrap_or(true),
        host.setting_f64("speed").unwrap_or(freecam::DEFAULT_SPEED),
    )
}

fn enter(host: &mut dyn ClientHost) {
    let Some(camera) = host.camera() else { return };
    let view = VIEWS
        .iter()
        .position(|v| *v == host.camera_type())
        .unwrap_or(0) as u8;
    freecam::set_settings(settings(host));
    freecam::start(camera.position, camera.yaw, camera.pitch, view);
    host.set_camera_type(CameraType::ThirdPersonBack);
}

fn exit(host: &mut dyn ClientHost) {
    if let Some(view) = freecam::stop() {
        host.set_camera_type(VIEWS[usize::from(view).min(VIEWS.len() - 1)]);
    }
}

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        r.register_key_mapping(
            KeyRegistration::new(TOGGLE, "key.keyboard.u").label("key.freecam_plus.toggle"),
        );
        let specs = [
            SettingSpec::toggle("noclip", "freecam_plus.setting.noclip", true),
            SettingSpec::range(
                "speed",
                "freecam_plus.setting.speed",
                freecam::MIN_SPEED,
                freecam::MAX_SPEED,
                freecam::SPEED_STEP,
                freecam::DEFAULT_SPEED,
            ),
        ];
        for spec in specs {
            if let Err(error) = r.register_setting(spec) {
                eprintln!("[{ID}] setting refused: {error:?}");
            }
        }
    }

    fn input_key(&self, host: &mut dyn ClientHost, event: &mut InputKeyEvent<'_>) {
        let pressed = match event.action {
            KeyAction::Press => true,
            KeyAction::Release => false,
            _ => return,
        };
        if freecam::active()
            && let Some(input) = MOVES.iter().position(|m| *m == event.key)
            && (!pressed || host.playing())
        {
            freecam::hold(input, pressed);
        }
    }

    fn client_tick_post(&self, host: &mut dyn ClientHost, _: &mut ClientTickPostEvent) {
        while host.consume_click(ID, TOGGLE) {
            if freecam::active() {
                exit(host);
            } else {
                enter(host);
            }
        }
        if !freecam::active() {
            return;
        }
        if host.camera_entity().is_none() {
            exit(host);
            return;
        }
        host.set_camera_type(CameraType::ThirdPersonBack);
        let current = settings(host);
        freecam::set_settings(current);
        let host = &*host;
        freecam::tick(|block| !current.noclip && !host.block_collision(block).is_empty());
    }

    fn movement_input(&self, _: &dyn ClientHost, event: &mut MovementInputEvent) {
        if freecam::active() {
            event.forward = 0.0;
            event.strafe = 0.0;
            event.jump = false;
            event.sneak = false;
            event.forward_key = false;
            event.backward_key = false;
            event.left_key = false;
            event.right_key = false;
            event.sprint = false;
        }
    }

    fn camera_pose(&self, _: &dyn ClientHost, event: &mut CameraPoseEvent<'_>) {
        if let Some((at, yaw, pitch)) = freecam::view(event.partial) {
            *event.position = at;
            event.yaw = yaw;
            event.pitch = pitch;
        }
    }

    fn mouse_scrolling(&self, host: &mut dyn ClientHost, event: &mut MouseScrollingEvent) -> Flow {
        let steps = event.accumulated[1];
        if steps == 0 {
            return Flow::Continue;
        }
        match freecam::wheel(steps as f32) {
            Some(changed) => {
                host.set_setting(ID, "speed", &changed.speed.to_string());
                Flow::Cancel
            }
            None => Flow::Continue,
        }
    }
}
