mod color;
mod video;

fn main() {
    let video_path = "/mnt/c/Users/Albert/Downloads/nyx_mouse.mp4";
    video::extract_frames(video_path);
}

