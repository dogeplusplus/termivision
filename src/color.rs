// use image;
// use image::imageops;

// use ansi_term::{ANSIString,Colour};

// const BLOCK: &str = "█";

// type ANSILine<'a> = Vec<ANSIString<'a>>;
// type ANSIImage<'a> = Vec<ANSILine<'a>>;


// pub fn image_to_ansi<'a>(img: &'a image::DynamicImage, dst_width: u32, dst_height: u32) -> ANSIImage {
//     let resized = image::DynamicImage::ImageRgba8(
//         imageops::resize(
//             img,
//             dst_width,
//             dst_height,
//             imageops::FilterType::Nearest
//         )
//     );

//     let pixels: Vec<ANSIString> = resized.to_rgb8().pixels().map(|x| {
//         Colour::RGB(x.0[0], x.0[1], x.0[2]).paint(BLOCK)
//     }).collect();

//     let mut results = ANSIImage::new();
//     for line in pixels.chunks(dst_width as usize) {
//         results.push(line.to_vec());
//     }
//     results
// }

// pub fn print_line(line: ANSILine) {
//     for c in line {
//         print!("{}", c);
//     }
//     print!("\n");
// }

// pub fn print_image(image: ANSIImage) {
//     for l in image {
//         print_line(l);
//     }
// }
