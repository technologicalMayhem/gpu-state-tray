use std::time::Duration;
use ksni::{Icon, Tray};

    fn main() {
        let service = ksni::TrayService::new(TrayIcon::new());
        let handle = service.handle();
        service.spawn();

        loop {
            handle.update(|icon: &mut TrayIcon| icon.update());
            std::thread::sleep(Duration::from_secs(2));
        }
    }

//Imports generated constants for the images
include!(concat!(env!("OUT_DIR"), "/const_images.rs"));

struct TrayIcon {
    is_powered: bool,
}

impl TrayIcon {
    fn new() -> Self {
        Self {
            is_powered: false,
        }
    }

    fn update(&mut self) {
        let state = std::fs::read_to_string("/sys/class/drm/card0/device/power_state").expect("Failed to read power state");
        if state.contains("D0") {
            self.is_powered = true;
        } else {
            self.is_powered = false;
        }
    }
}

impl Tray for TrayIcon {
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn title(&self) -> String {
        if self.is_powered { "GPU active" } else { "GPU inactive" }.into()
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        let data = if self.is_powered {
            IMAGE_ACTIVE.to_vec()
        } else {
            IMAGE_OFF.to_vec()
        };

        vec![
            Icon {
                height: 96,
                width: 96,
                data
            }
        ]
    }
}