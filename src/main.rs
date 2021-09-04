mod color;

fn main() {
    let image_path = "/mnt/c/Users/Albert/Downloads/fish_sam.jpeg";
    let img = image::open(image_path).unwrap();
    let result = color::image_to_ansi(&img, 200, 100);
    color::print_image(result);
}

