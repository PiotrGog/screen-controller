use opencv::{core, imgproc, prelude::*, videoio};

pub struct Brightness {
    camera: videoio::VideoCapture,
}

impl Brightness {
    pub fn new() -> opencv::Result<Brightness> {
        #[cfg(feature = "opencv-32")]
        let mut cam = videoio::VideoCapture::new_default(0)?; // 0 is the default camera
        #[cfg(not(feature = "opencv-32"))]
        let cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
        let opened = videoio::VideoCapture::is_opened(&cam)?;
        if !opened {
            panic!("Unable to open default camera!");
        }
        return Ok(Brightness { camera: cam });
    }

    pub fn get_brightness(&mut self) -> opencv::Result<u8> {
        let mut input_color_frame = core::Mat::default()?;
        let mut gray_frame = core::Mat::default()?;
        self.camera.read(&mut input_color_frame)?;
        imgproc::cvt_color(
            &input_color_frame,
            &mut gray_frame,
            imgproc::COLOR_RGB2GRAY,
            1,
        )?;

        let size = gray_frame.size()?;
        let mask = core::Mat::ones(size.height, size.width, core::CV_8U)?;
        let mean = core::mean(&gray_frame, &mask)?;
        let mean = mean[0] as u8;

        return Ok(mean);
    }
}
