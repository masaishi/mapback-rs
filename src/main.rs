//! A command-line tool for generating unzoomed level images from map tile images.
//!
//! This tool takes a folder containing map tile images at different zoom levels and generates
//! unzoomed level images by combining the tiles from the last available zoom level.

use clap::Parser;
use image::{imageops::resize, imageops::FilterType, io::Reader as ImageReader, ImageBuffer, Rgba, GenericImage};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{create_dir_all, read_dir},
    path::Path,
    process,
};

/// Command-line arguments for the map tile unzooming tool.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder path of the map tile data.
    folder: String,

    /// Most detailed zoom level (reduce zoom level until found).
    #[clap(long, default_value = "24")]
    max_zoom: u8,

    /// Least detailed zoom level.
    #[clap(long, default_value = "0")]
    min_zoom: u8,
}

fn main() {
    let args = Args::parse();

    if !Path::new(&args.folder).exists() {
        eprintln!("Folder does not exist: {}", args.folder);
        process::exit(1);
    }

    let mut zoom_level = find_last_zoom_level(&args.folder, args.max_zoom, args.min_zoom);
    println!("Starting zoom level: {}", zoom_level);
    
    while zoom_level > args.min_zoom {
        let image_paths = collect_image_paths(&args.folder, zoom_level);
        println!("Total PNG files at zoom level {}: {}", zoom_level, image_paths.len());

        let output_zoom_level = zoom_level - 1;
        let output_dir = format!("{}/{}", args.folder, output_zoom_level);
        create_dir_all(&output_dir).unwrap();

        let progress_bar = create_progress_bar(image_paths.len() as u64);

        println!("Generating {} level images...", output_zoom_level);
        for image_path in image_paths {
            process_image_path(&image_path, &args, output_zoom_level, zoom_level, &progress_bar);
        }

        progress_bar.finish_with_message("Done!");

        zoom_level -= 1;
    }
}

fn find_last_zoom_level(folder: &str, max_zoom: u8, min_zoom: u8) -> u8 {
    (min_zoom..=max_zoom)
        .rev()
        .find(|&zoom| Path::new(&format!("{}/{}", folder, zoom)).exists())
        .unwrap_or(min_zoom)
}

fn collect_image_paths(folder: &str, zoom_level: u8) -> Vec<String> {
    let mut image_paths = Vec::new();
    let zoom_dir = format!("{}/{}", folder, zoom_level);
    if let Ok(x_entries) = read_dir(&zoom_dir) {
        for x_entry in x_entries.filter_map(Result::ok) {
            if let Ok(y_entries) = read_dir(x_entry.path()) {
                for y_entry in y_entries.filter_map(Result::ok) {
                    let path = y_entry.path();
                    if let Some("png") = path.extension().and_then(|ext| ext.to_str()) {
                        if let Some(image_path) = path.strip_prefix(folder).ok().and_then(|p| p.to_str()) {
                            image_paths.push(image_path.to_string());
                        }
                    }
                }
            }
        }
    }
    image_paths
}

fn parse_image_path(image_path: &str) -> (u32, u32) {
    let parts: Vec<&str> = image_path.split('/').collect();
    let x = parts[1].parse().unwrap();
    let y = parts[2].parse().unwrap();
    (x, y)
}

fn create_progress_bar(total: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(total);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .expect("Failed to set progress bar template")
            .progress_chars("#>-"),
    );
    progress_bar
}

fn fill_transparent(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32, width: u32, height: u32) {
    for i in 0..width {
        for j in 0..height {
            image.put_pixel(x + i, y + j, Rgba([0, 0, 0, 0]));
        }
    }
}

fn process_image_path(
    image_path: &str,
    args: &Args,
    output_zoom_level: u8,
    zoom_level: u8,
    progress_bar: &ProgressBar,
) {
    if let Some((x, y)) = image_path.strip_suffix(".png").map(parse_image_path) {
        let (back_x, back_y) = (x / 2, y / 2);
        let back_path = format!("{}/{}/{}/{}.png", args.folder, output_zoom_level, back_x, back_y);

        let mut output_image = ImageBuffer::new(512, 512);

        for i in 0..2 {
            for j in 0..2 {
                let path = format!("{}/{}/{}/{}.png", args.folder, zoom_level, back_x * 2 + i, back_y * 2 + j);
                if Path::new(&path).exists() {
                    let image = ImageReader::open(path).unwrap().decode().unwrap();
                    let resized_image = resize(&image, 256, 256, FilterType::Lanczos3);
                    output_image.copy_from(&resized_image, i * 256, j * 256).unwrap();
                } else {
                    fill_transparent(&mut output_image, i * 256, j * 256, 256, 256);
                }
            }
        }

        create_dir_all(Path::new(&back_path).parent().unwrap()).unwrap();
        output_image.save(&back_path).unwrap();
    }
    progress_bar.inc(1);
}
