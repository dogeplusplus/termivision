use std::io;
use std::io::Write;
use term_size;
use opencv::imgproc;
use opencv::imgproc::{INTER_CUBIC};
use opencv::core::{Mat,Vec3, Size, CV_8UC3,Scalar};
use ansi_term::{Colour,ANSIString};
use opencv::videoio;
use opencv::prelude::{VideoCaptureTrait,MatTraitConst};

const BLOCK: &str = "█";

pub fn extract_frames(video_path: &str) {
    let mut video_capture = videoio::VideoCapture::default().unwrap();
    video_capture.open_file(video_path, videoio::CAP_FFMPEG).unwrap();

    let mut frame = Mat::default();

    loop {
        match video_capture.read(&mut frame).unwrap() {
            true => (),
            false => break,
        };


        let mut height = 50;
        let mut width = 70;
        if let Some((w,h)) = term_size::dimensions() {
            height = h as i32 - 1;
            width = w as i32;
        }

        let size = Size{ height: height, width: width };
        let scalar = Scalar{0: [0.0, 0.0, 0.0, 0.0]};
        let mut resized = Mat::new_size_with_default(size, CV_8UC3, scalar).unwrap();
        imgproc::resize(
            &frame, &mut resized, size, 0.0, 0.0, INTER_CUBIC
        ).expect("Resize failed.");


        let height = resized.rows();
        let width = resized.cols();

        for h in 0..height {
            for w in 0..width {
                let rgb = *resized.at_2d(h, w).unwrap();
                let ansi_rgb = pixel_to_ansi(rgb);
                print!("{}", ansi_rgb);
            }
        }
        io::stdout().flush().unwrap();
    }
}

fn pixel_to_ansi(rgb: Vec3<u8>) -> ANSIString<'static> {
    Colour::RGB(rgb.0[2], rgb.0[1], rgb.0[0]).paint(BLOCK)
}
