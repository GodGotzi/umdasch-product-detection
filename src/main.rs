use anyhow::Result;

use opencv::{
    prelude::*,
    videoio,
    highgui,
};


fn main() -> Result<()> {
    

    highgui::named_window("video", highgui::WINDOW_FULLSCREEN)?;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();

    loop {
        cam.read(&mut frame)?;
        
        highgui::imshow("Frame", &frame)?;
        
        if highgui::wait_key(1)? == 113 {
            break;
        }
    }


    Ok(())
}
