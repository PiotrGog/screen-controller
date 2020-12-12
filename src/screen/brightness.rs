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

    pub fn set_screen_bright(&mut self, screen_name: &str, new_value: f32) -> bool {
        if let Some(bright_value) = self.screen_brightness.get_mut(screen_name) {
            if controller::set_brightness(screen_name, new_value / 255 as f32) {
                *bright_value = new_value;
                return true;
            };
        }
        return false;
    }

    pub fn set_screens_bright(&mut self, new_value: f32) -> bool {
        let mut result = true;
        for (screen_name, bright_value) in &mut self.screen_brightness {
            if controller::set_brightness(&screen_name, new_value / self.maximum_bright as f32) {
                *bright_value = new_value;
            } else {
                result |= false;
            }
        }

        return result;
    }
}