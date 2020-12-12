use opencv;
mod camera;
mod screen;

fn main() -> opencv::Result<()> {
    let maximum_bright_threshold = 128;
    let mut camera = camera::brightness::Brightness::new()?;
    let mut screen = screen::brightness::Brightness::new(maximum_bright_threshold);

    screen.add_screen("eDP1");
    screen.add_screen("HDMI2");
    loop {
        let camera_brightness = camera.get_brightness()?;
        screen.set_screens_bright(camera_brightness as f32);
    }
}
