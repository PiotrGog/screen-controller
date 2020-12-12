use std;

use crate::screen::controller;

pub struct Brightness {
    maximum_bright: u8,
    maximum_history_length: usize,
    screen_brightness: std::collections::HashMap<String, f32>,
    screen_brightness_history: std::collections::HashMap<String, std::collections::LinkedList<f32>>,
}

impl Brightness {
    pub fn new(maximum_bright: u8) -> Brightness {
        return Brightness {
            maximum_bright: maximum_bright,
            maximum_history_length: 10,
            screen_brightness: std::collections::HashMap::new(),
            screen_brightness_history: std::collections::HashMap::new(),
        };
    }

    pub fn add_screen(&mut self, screen_name: &str) {
        self.screen_brightness
            .insert(String::from(&screen_name[..]), 1 as f32);

        let mut history = std::collections::LinkedList::new();
        history.push_back(1.0);
        self.screen_brightness_history
            .insert(String::from(&screen_name[..]), history);
    }

    pub fn remove_screen(&mut self, screen_name: &str) {
        self.screen_brightness.remove(&screen_name[..]);
        self.screen_brightness_history.remove(&screen_name[..]);
    }

    pub fn autodetection_screens(&mut self) {
        let active_monitors = controller::active_monitors();
        for monitor in active_monitors {
            self.add_screen(&monitor[..]);
        }
    }

    pub fn set_screen_bright(&mut self, screen_name: &str, new_value: u8) -> bool {
        let brightness = new_value as f32 / self.maximum_bright as f32;
        let brightness = if brightness > 1.0 {
            1.0
        } else if brightness < 0.3 {
            0.3
        } else {
            brightness
        };
        if let Some(bright_value) = self.screen_brightness.get_mut(screen_name) {
            if let Some(history) = self.screen_brightness_history.get_mut(screen_name) {
                history.push_back(brightness);

                if history.len() > self.maximum_history_length {
                    history.pop_front();
                    let brightness =
                        history.iter().sum::<f32>() / self.maximum_history_length as f32;
                    if controller::set_brightness(screen_name, brightness) {
                        *bright_value = brightness;
                        return true;
                    }
                }
            }
        }

        return false;
    }

    pub fn set_screens_bright(&mut self, new_value: u8) -> bool {
        let mut result = true;
        let screens = self
            .screen_brightness
            .iter()
            .map(|(key, _)| {
                return key.clone();
            })
            .collect::<Vec<String>>();
        for screen_name in screens {
            if !self.set_screen_bright(&screen_name[..], new_value) {
                result = false;
            }
        }

        return result;
    }
}
