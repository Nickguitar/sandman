use users::get_current_uid;

/// Running configuration that a toggle implies
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ToggleImplication {
    pub env: Vec<String>,
    pub volumes: Vec<String>,
    pub devices: Vec<String>,
    pub args: Vec<String>,
}

/// All allowed and expected toggles
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Toggles {
    pub x11: ToggleImplication,
    pub wayland: ToggleImplication,
    pub dri: ToggleImplication,
    pub ipc: ToggleImplication,
    pub pulseaudio: ToggleImplication,
    pub dbus: ToggleImplication,
    pub net: ToggleImplication,
    pub uidmap: ToggleImplication,
    pub hub: ToggleImplication,
}

impl Toggles {

    /// Helper function to get an environment variable's value
    fn env(var_name: &str) -> String {
        std::env::var(var_name).unwrap_or_default()
    }

    /// Returns the specific configuration for the toggles compiled at runtime
    pub fn get_toggles() -> Toggles {
        let x11 = ToggleImplication {
            env: vec![
                String::from("DISPLAY"),
                String::from("XCURSOR_THEME"),
                String::from("XCURSOR_SIZE"),
            ],
            volumes: vec![String::from("/tmp/.X11-unix:/tmp/.X11-unix")],
            devices: vec![],
            args: vec![],
        };

        let wayland = ToggleImplication {
            env: vec![String::from("WAYLAND_DISPLAY")],
            volumes: vec![String::from(format!("{}/{}:{}/{}",
                                            Toggles::env("XDG_RUNTIME_DIR"),
                                            Toggles::env("WAYLAND_DISPLAY"),
                                            Toggles::env("XDG_RUNTIME_DIR"),
                                            Toggles::env("WAYLAND_DISPLAY"))),
            ],
            devices: vec![],
            args: vec![],
        };

        let dri = ToggleImplication {
            env: vec![],
            volumes: vec![],
            devices: vec![String::from("/dev/dri")],
            args: vec![],
        };

        let ipc = ToggleImplication {
            env: vec![],
            volumes: vec![],
            devices: vec![],
            args: vec![String::from("--ipc"), String::from("host")],
        };

        let pulseaudio = ToggleImplication {
            env: vec![String::from("XDG_RUNTIME_DIR")],
            volumes: vec![
                String::from("/etc/machine-id:/etc/machine-id:ro"),
                String::from(format!("{}/pulse/native:{}/pulse/native",
                                    Toggles::env("XDG_RUNTIME_DIR"),
                                    Toggles::env("XDG_RUNTIME_DIR"))),
            ],
            devices: vec![],
            args: vec![],
        };

        let current_uid = get_current_uid();
        let first_uid = current_uid + 1;
        let last_uid = 65536 - current_uid;

        let uidmap = ToggleImplication {
            env: vec![],
            volumes: vec![],
            devices: vec![],
            args: vec![
                // TODO Calculate this based on current uid
                String::from("--uidmap"),
                String::from(format!("{}:0:1", current_uid)),
                String::from("--uidmap"),
                String::from(format!("0:1:{}", current_uid)),
                String::from("--uidmap"),
                String::from(format!("{}:{}:{}", first_uid, first_uid, last_uid)),
                String::from("--user"),
                String::from(format!("{}", current_uid)),
            ],
        };

        let dbus = ToggleImplication {
            env: vec![String::from(format!("DBUS_SESSION_BUS_ADDRESS=unix:path={}/bus",
                                           Toggles::env("XDG_RUNTIME_DIR"))),
            ],
            volumes: vec![String::from(format!("{}/bus:{}/bus",
                                               Toggles::env("XDG_RUNTIME_DIR"),
                                               Toggles::env("XDG_RUNTIME_DIR"))),
            ],
            devices: vec![],
            args: vec![],
        };

        let net = ToggleImplication {
            env: vec![],
            volumes: vec![],
            devices: vec![],
            args: vec![String::from("--network"), String::from("slirp4netns")],
        };

        let hub = ToggleImplication {
            env: vec![],
            volumes: vec![
                String::from("/home/nich0las/sandman/hub/:/home/user/hub/"),
            ],
            devices: vec![],
            args: vec![],
        };

        Toggles {
            x11: x11,
            wayland: wayland,
            dri: dri,
            ipc: ipc,
            pulseaudio: pulseaudio,
            dbus: dbus,
            net: net,
            uidmap: uidmap,
            hub: hub,
        }
    }

}
