#![allow(dead_code)]
mod color;
mod matrix;
mod pipeline;
mod program;

use std::{fs::File, io::BufWriter, path::Path};

use matrix::Vector3;
use pipeline::frame::Frame;
use program::Program;

fn main() {
    let mut frame: Frame<f32, 200, 300> = Frame::new();

    let program = Box::from(Program {});

    let verts = vec![
        Vector3::new3(-1f32, -1f32, 0f32),
        Vector3::new3(1f32, -1f32, 0f32),
        Vector3::new3(0f32, 1f32, 0f32),
    ];

    pipeline::draw(&mut frame, program, &verts);

    let bytes = frame.byte_sequence();

    write_to_png("output.png", &bytes);
}

fn write_to_png(path: &str, bytes_rgba: &[u8]) {
    let p = Path::new(path);
    let file = File::create(p).unwrap();
    let ref mut writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(writer, 300, 200);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8));
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    let source_chromaticities = png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&bytes_rgba).unwrap();
}
