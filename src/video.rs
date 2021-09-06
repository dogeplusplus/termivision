use opencv::imgproc;
use opencv::imgproc::{INTER_CUBIC};
use opencv::core::{Mat,Vec3, Size, CV_8UC3,Scalar};
use ansi_term::{Colour,ANSIString};
use opencv::videoio;
use opencv::prelude::{VideoCaptureTrait,MatTraitConst};

type TermLine<'a> = Vec<ANSIString<'a>>;
type TermImage<'a> = Vec<TermLine<'a>>;

const BLOCK: &str = "█";


pub fn extract_frames(video_path: &str) -> Vec<TermImage>{

    let mut video_capture = videoio::VideoCapture::default().unwrap();
    video_capture.open_file(video_path, videoio::CAP_FFMPEG).unwrap();

    let mut frame = Mat::default();

    let mut ansi_video = vec![];
    let mut counter = 0;

    loop {
        match video_capture.read(&mut frame).unwrap() {
            true => (),
            false => break,
        };

        println!("Processing Frame: {}", counter);
        let size = Size{ height: 200, width: 200};
        let scalar = Scalar{0: [0.0, 0.0, 0.0, 0.0]};
        let mut resized = Mat::new_size_with_default(size, CV_8UC3, scalar).unwrap();
        let resize_success = imgproc::resize(&frame, &mut resized, size, 0.0, 0.0, INTER_CUBIC);

        match resize_success {
            Ok(_) => (),
            Err(_) => break,
        };

        let height = resized.rows();
        let width = resized.cols();

        let mut image = TermImage::new();

        for h in 0..height {
            let mut row = TermLine::new();
            for w in 0..width {
                let rgb = *resized.at_2d(h, w).unwrap();
                let ansi_rgb = pixel_to_ansi(rgb);
                row.push(ansi_rgb);
            }
            image.push(row);
        }
        ansi_video.push(image);
        counter += 1;
    }
    ansi_video
}

fn pixel_to_ansi(rgb: Vec3<u8>) -> ANSIString<'static> {
    Colour::RGB(rgb.0[2], rgb.0[1], rgb.0[0]).paint(BLOCK)
}

fn cout_line(line: TermLine) {
    for c in line {
        print!("{}", c);
    }
    print!("\n");
}


pub fn cout_image(image: TermImage) {
    for l in image {
        cout_line(l);
    }
}
