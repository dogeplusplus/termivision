mod video;

fn main() {
    let video_path = "/mnt/c/Users/Albert/Downloads/nyx_mouse.mp4";
    let ansi_video = video::extract_frames(video_path);
    for frame in ansi_video {
        video::cout_image(frame.to_vec());
    }
}

