use std;

use crate::screen::controller;

pub struct Brightness {
    screen_brightness: std::collections::HashMap<String, f32>,
    maximum_bright: u8,
}

impl Brightness {
    pub fn new(maximum_bright: u8) -> Brightness {
        return Brightness {
            screen_brightness: std::collections::HashMap::new(),
            maximum_bright: maximum_bright,
        };
    }

    pub fn add_screen(&mut self, screen_name: &str) {
        self.screen_brightness
            .insert(String::from(&screen_name[..]), 1 as f32);
    }

    pub fn remove_screen(&mut self, screen_name: &str) {
        self.screen_brightness.remove(&screen_name[..]);
    }

    pub fn autodetection_screens(&mut self) {
        let active_monitors = controller::active_monitors();
        for monitor in active_monitors {
            self.add_screen(&monitor[..]);
        }
    }

    pub fn set_screen_bright(&mut self, screen_name: &str, new_value: u8) -> bool {
        let brightness = new_value as f32 / self.maximum_bright as f32;

        if let Some(bright_value) = self.screen_brightness.get_mut(screen_name) {
            if controller::set_brightness(
                screen_name,
                if brightness > 1.0 { 1.0 } else { brightness },
            ) {
                *bright_value = brightness;
                return true;
            };
        }
        return false;
    }

    pub fn set_screens_bright(&mut self, new_value: u8) -> bool {
        let mut result = true;
        let brightness = new_value as f32 / self.maximum_bright as f32;

        for (screen_name, bright_value) in &mut self.screen_brightness {
            if controller::set_brightness(
                &screen_name,
                if brightness > 1.0 { 1.0 } else { brightness },
            ) {
                *bright_value = brightness;
            } else {
                result |= false;
            }
        }

        return result;
    }
}
