use opencv;
mod camera;

fn main() -> opencv::Result<()> {
    let mut camera = camera::brightness::Brightness::new()?;

    loop {
        let camera_brightness = camera.get_brightness()?;
        println!("{}", camera_brightness);
    }
}
