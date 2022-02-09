use std::{fs::File, thread};

use gifski::{Collector, CatResult, Settings, progress::ProgressBar, Writer};
use imgref::ImgVec;
use rgb::RGBA8;


pub type BinResult<T, E = Box<dyn std::error::Error + Send + Sync>> = Result<T, E>;


fn collect(
    frames: Vec<ImgVec<RGBA8>>,
    collector: &mut Collector,
    frames_per_sec: f64,
) -> CatResult<()> {
    println!("collecting {} frames", frames.len());

    for (i, frame) in frames.into_iter().enumerate() {
        collector.add_frame_rgba(i, frame, i as f64 / frames_per_sec)?;
    }
    Ok(())
}

pub fn write(writer: Writer, output_path: &str, total: u64 ) -> BinResult<()>{

    let mut progress = ProgressBar::new(total);
    let file = File::create(output_path)
        .map_err(|e| println!("Can't write to {}: {}", output_path, e))
        .expect("cannot create file");
    writer.write(file, &mut progress)?;
    Ok(())
}

pub fn images_to_gif(
    frames: Vec<ImgVec<RGBA8>>, frames_per_sec: f64, output_path: &str) -> BinResult<()> {
    let (mut collector, writer) = gifski::new(Settings::default()).unwrap();

    let total = frames.len() as u64;

    let decode_thread = thread::Builder::new()
        .name("decode".into())
        .spawn(move || {
            collect(frames, &mut collector, frames_per_sec).expect("collect went well");
        })?;

    write(writer, output_path, total)?;
    decode_thread.join().map_err(|_| "thread died?")?;
    Ok(())
}
