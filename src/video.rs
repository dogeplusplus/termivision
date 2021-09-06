use opencv::core::{Mat,Vec3};
use ansi_term::{Colour,ANSIString};
use opencv::videoio;
use opencv::prelude::{VideoCaptureTrait,MatTraitConst,MatTrait};

const BLOCK: &str = "█";


pub fn extract_frames(video_path: &str) -> Vec<Vec<Vec<ANSIString>>>{

    let mut video_capture = videoio::VideoCapture::default().unwrap();
    video_capture.open_file(video_path, videoio::CAP_FFMPEG).unwrap();

    let mut frame = Mat::default();

    let mut ansi_video = vec![];
    let mut counter = 0;
    while let Ok(_) = video_capture.read(&mut frame) {
        println!("Processing Frame: {}", counter);
        let height = frame.rows();
        let width = frame.cols();

        let mut rows = vec![];

        for h in 0..height {
            let mut row = vec![];
            for w in 0..width {
                let rgb = *frame.at_2d(h, w).unwrap();
                let ansi_rgb = pixel_to_ansi(rgb);
                row.push(ansi_rgb);
            }
            rows.push(row);
        }
        ansi_video.push(rows);
        counter += 1;
    }
    ansi_video
}

fn pixel_to_ansi(rgb: Vec3<u8>) -> ANSIString<'static> {
    Colour::RGB(rgb.0[0], rgb.0[1], rgb.0[2]).paint(BLOCK)
}
