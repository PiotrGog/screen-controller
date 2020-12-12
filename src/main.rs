use opencv;
mod camera;
mod screen;

fn main() -> opencv::Result<()> {
    let maximum_bright_threshold = 110;
    let mut camera = camera::brightness::Brightness::new()?;
    let mut screen = screen::brightness::Brightness::new(maximum_bright_threshold);

    screen.autodetection_screens();

    loop {
        let camera_brightness = camera.get_brightness()?;
        screen.set_screens_bright(camera_brightness as f32);
    }
}
