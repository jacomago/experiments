use std::{
    fs::{self, File},
    path::PathBuf,
    thread,
};

use gifski::{
    progress::{ProgressBar, ProgressReporter},
    CatResult, Collector, Settings,
};
use video_capture::BinResult;

fn main() {
    folder_gif(
        "./assets/images/gif/output/cube_flow",
        25.0,
        "cube_flow1.gif",
    )
    .expect("expected a folder of images");
}

fn get_frames(path: &str) -> Vec<PathBuf> {
    println!("Beginning directory read for {:?}", path);
    let dir = fs::read_dir(path).expect("path should exist");
    let mut output = Vec::new();
    for dir_entry in dir {
        let entry = dir_entry.expect("directory not empty");
        let entry_path = entry.path();
        if !entry_path.is_dir() {
            output.push(entry_path);
        }
    }
    output
}

fn collect(
    frames: &mut Vec<PathBuf>,
    collector: &mut Collector,
    frames_per_sec: f64,
) -> CatResult<()> {
    println!("collecting {} frames", frames.len());

    for (i, frame) in frames.drain(..).enumerate() {
        collector.add_frame_png_file(i, frame, i as f64 / frames_per_sec)?;
    }
    Ok(())
}

fn folder_gif(path: &str, frames_per_sec: f64, output_path: &str) -> BinResult<()> {
    let (mut collector, writer) = gifski::new(Settings::default()).unwrap();

    let mut frames = get_frames(path);
    let total = frames.len() as u64;

    let decode_thread = thread::Builder::new()
        .name("decode".into())
        .spawn(move || {
            collect(&mut frames, &mut collector, frames_per_sec).expect("collect went well");
        })?;

    let mut progress = ProgressBar::new(total);
    let file = File::create(output_path)
        .map_err(|e| println!("Can't write to {}: {}", output_path, e))
        .expect("cannot create file");
    writer.write(file, &mut progress)?;

    decode_thread.join().map_err(|_| "thread died?")?;
    progress.done(&format!("gifski created {}", output_path));
    Ok(())
}
