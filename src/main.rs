mod video;

fn main() {
    let video_path = "example/woodcock.gif";
    video::extract_frames(video_path);
}

