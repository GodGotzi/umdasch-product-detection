
use opencv::prelude::*;

fn main() {
    let vid = VideoCapture::new(0).unwrap();

    while true {
        let mut frame: Mat = Mat::default();
        
        vid.read(&mut frame).unwrap();

        imshow("frame", &frame).unwrap();
    }


    println!("Hello, world!");
}
