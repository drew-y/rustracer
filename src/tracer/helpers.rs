use indicatif::{ProgressBar, ProgressStyle};
use std::thread;

pub type RenderThread = thread::JoinHandle<Vec<u8>>;

pub fn make_progress_bar(msg: &str, total: i32) -> ProgressBar {
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {msg} [{bar:50.cyan/blue}] ({eta})")
            .progress_chars("#>-"),
    );
    pb.set_message(msg);
    pb.set_position(0);
    pb
}
