use image::{DynamicImage, ImageFormat};
use reqwest;
use std::path::{Path};
use tokio;
use clap::{Parser, ArgGroup};
use env_logger;
use log::{debug, error, info};

mod models;
mod api;

use api::fetch_avatar_profile_card;

#[derive(Parser)]
#[clap(group = ArgGroup::new("ArgGroup").required(true).multiple(false))]
struct Opts {
    #[arg(short, long, help = "The user ID", group = "ArgGroup")]
    cid: Option<i64>,

    #[clap(short = 'a', long, group = "ArgGroup", help = "The avatar name")]
    avatar_name: Option<String>,

    #[arg(short, long, help = "The output file name")]
    output: Option<String>,

    #[clap(short, long, help = "Enable verbose logging")]
    verbose: bool,

    #[arg(short, long, help = "Grid hex color. Example: 'ececec'", default_value = "ececec")]
    grid_color: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let opts: Opts = Opts::parse();

    if opts.verbose {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    debug!("Verbose logging enabled");

    let cid = match opts.avatar_name {
        Some(ref avatar_name) => api::get_user_id_from_avatar_name(&avatar_name).await?,
        None => {
            match opts.cid {
                Some(cid_value) => cid_value,
                None => {
                    eprintln!("Either --avatar_name or --cid must be provided!");
                    std::process::exit(1);
                }
            }
        }
    };


    let avatar_card = fetch_avatar_profile_card(cid).await?;

    info!("Avatar name: {}", avatar_card.avname);

    // if badges is empty, print a message and return
    if avatar_card.badges.is_empty() {
        println!("No badges found!");
        return Ok(());
    }

    info!("Found {} badges", avatar_card.badges.len());

    let output_file_name = match opts.output {
        Some(name) => name,
        None => format!("canvas-{}.png", avatar_card.avname)
    };

    let output_path = Path::new(output_file_name.as_str());

    let image_format = match output_path.extension() {
        Some(ext) if ext == "png" => ImageFormat::Png,
        Some(ext) if ext == "jpg" || ext == "jpeg" => ImageFormat::Jpeg,
        _ => {
            log::error!("Unsupported output file format. Please use either .png or .jpg/.jpeg.");
            return Ok(());
        }
    };

    let mut base_image = tile_image(&opts.grid_color)?;

    for (_, badge_info) in &avatar_card.badges {
        let badge_image = reqwest::get(&badge_info.image_url).await?.bytes().await?;

        let image_format = image::guess_format(&badge_image)?;
        let badge_dynamic_image = match image_format {
            ImageFormat::Gif => {
                debug!("Loading GIF badge {}", badge_info.to_id_string());
                // Handle GIFs, maybe convert them or handle frames differently
                image::load_from_memory_with_format(&badge_image, image::ImageFormat::Gif)?
            },
            ImageFormat::Png => {
                debug!("Loading PNG badge {}", badge_info.to_id_string());
                // Handle PNGs
                image::load_from_memory_with_format(&badge_image, image::ImageFormat::Png)?
            },
            // Add cases for other formats if needed
            _ => {
                error!("Unexpected image format for badge {}. Using default loader.", badge_info.to_id_string());
                image::load_from_memory(&badge_image)?
            }
        }.to_rgba8();

        let x = badge_info.xloc;
        let y = badge_info.yloc % 100;

        debug!("Overlaying badge {} at ({}, {}) ({}, {})", badge_info.to_id_string(), x, y, badge_info.xloc, badge_info.yloc);

        image::imageops::overlay(&mut base_image, &badge_dynamic_image, x, y);
    }

    base_image.save_with_format(output_path, image_format)?;

    info!("Saved image to {}", output_path.display());

    Ok(())
}

fn tile_image(grid_color_hex: &str) -> Result<image::DynamicImage, image::ImageError> {
    // Assuming a default color for the grid and the grid lines
    let mut grid_color = [0xEC, 0xEC, 0xEC, 0xFF];
    let mut grid_lines_color = [0xD4, 0xD4, 0xD4, 0xFF];

    // Strip the # from the beginning if present
    let color_str = if grid_color_hex.starts_with('#') {
        &grid_color_hex[1..]
    } else {
        grid_color_hex
    };

    // Parse provided hex color
    if color_str.len() == 6 {
        if let Ok(decoded) = hex::decode(color_str) {
            grid_color = [decoded[0], decoded[1], decoded[2], 0xFF];

            // Check brightness
            let brightness = 0.299 * (grid_color[0] as f32)
                + 0.587 * (grid_color[1] as f32)
                + 0.114 * (grid_color[2] as f32);

            if brightness < 128.0 {
                // If the color is dark, lighten the grid lines
                grid_lines_color = [
                    grid_color[0].saturating_add(40),
                    grid_color[1].saturating_add(40),
                    grid_color[2].saturating_add(40),
                    0xFF
                ];
            } else {
                // If the color is light, darken the grid lines
                grid_lines_color = [
                    grid_color[0].saturating_sub(24),
                    grid_color[1].saturating_sub(24),
                    grid_color[2].saturating_sub(24),
                    0xFF
                ];
            }
        }
    }

    let mut image = image::RgbaImage::new(440, 100);

    for pixel in image.pixels_mut() {
        *pixel = image::Rgba(grid_color);
    }

    // Draw dashed grid lines with grid lines color
    for x in (0..438).step_by(20) {  // Adjusted to avoid overflowing the width
        for y in 0..100 {
            if y % 10 < 6 {
                // Vertical lines 1.5px wide
                image.put_pixel(x, y, image::Rgba(grid_lines_color));
                image.put_pixel(x + 1, y, image::Rgba(grid_lines_color));
            }
        }
    }
    for y in (0..94).step_by(20) {  // Adjusted to avoid overflowing the height
        for x in 0..440 {
            if x % 10 < 6 {
                // Horizontal lines 1.5px wide
                image.put_pixel(x, y, image::Rgba(grid_lines_color));
                image.put_pixel(x, y + 1, image::Rgba(grid_lines_color));
            }
        }
    }

    Ok(DynamicImage::ImageRgba8(image))
}
