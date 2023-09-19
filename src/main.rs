use image::{AnimationDecoder, DynamicImage, ImageFormat, RgbaImage};
use image::codecs::gif::GifDecoder;
use reqwest;
use std::path::{Path};
use std::sync::Arc;
use tokio;
use clap::{Parser, ArgGroup};
use env_logger;
use log::{debug, error, info};

mod models;
mod api;

use api::fetch_avatar_profile_card;
use crate::models::BadgeInfo;

#[derive(Parser)]
#[clap(group = ArgGroup::new("ArgGroup").required(true).multiple(false))]
#[clap(author = "Toyz", version, about, long_about = None, name = "Badge Canvas Generator")]
struct Opts {
    #[arg(short='c', long, help = "The user ID", group = "ArgGroup")]
    cid: Option<i64>,

    #[arg(short='a', long, group = "ArgGroup", help = "The avatar name")]
    avatar_name: Option<String>,

    #[arg(short='o', long, help = "The output file name")]
    output: Option<String>,

    #[arg(short='v', long, help = "Enable verbose logging")]
    verbose: bool,

    #[arg(short='g', long, help = "Grid hex color. Example: 'ececec'", default_value = "ececec")]
    grid_color: String,

    #[arg(short='j', long, help = "Number of concurrent tasks. Defaults to number of cores. Use 'auto' for default behavior.", default_value = "auto")]
    concurrency: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");

    let opts: Opts = Opts::parse();

    if opts.verbose {
        std::env::set_var("RUST_LOG", "debug");
    } else {
    }
    env_logger::init();

    debug!("Verbose logging enabled");

    let cid = match opts.avatar_name {
        Some(ref avatar_name) => api::get_user_id_from_avatar_name(&avatar_name).await?,
        None => {
            match opts.cid {
                Some(cid_value) => cid_value,
                None => {
                    return Err("No avatar name or user ID provided".into());
                }
            }
        }
    };

    let default_concurrency = num_cpus::get();
    let concurrency: usize = if opts.concurrency == "auto" {
        default_concurrency
    } else {
        opts.concurrency.parse().unwrap_or(default_concurrency)
    };


    let avatar_card = fetch_avatar_profile_card(cid).await?;

    info!("Avatar name: {}", avatar_card.avname);

    // if badges is empty, print a message and return
    if avatar_card.badges.is_empty() {
        return Err("No badges found".into());
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
            return Err("Invalid output file extension. Supported extensions are: png, jpg, jpeg".into());
        }
    };

    let mut base_image = tile_image(&opts.grid_color)?;


    let mut tasks = Vec::new();
    let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
    for (_, badge_info) in &avatar_card.badges {
        let badge_info_clone = badge_info.clone(); // Clone the badge info
        let sema_clone = Arc::clone(&semaphore); // Clone the Arc outside of the spawned block

        let handle = tokio::spawn(async move {
            let permit = sema_clone.acquire().await;
            // Drop the permit when the task is done
            let _permit_guard = permit;

            download_and_process(badge_info_clone).await
        });
        tasks.push(handle);
    }


    for task in tasks {
        // 3. Gather results and overlay
        match task.await {
            Ok(Ok(data)) => {
                let (badge_dynamic_image, id, (x, y), (badge_xloc, badge_yloc)) = data;
                debug!("Overlaying badge {} at Cur: ({}, {}) Org: ({}, {})", id, x, y, badge_xloc, badge_yloc);
                image::imageops::overlay(&mut base_image, &badge_dynamic_image, x, y);
            },
            Ok(Err(e)) => {
                // handle download/process error
                error!("Error processing image: {}", e);
            },
            Err(e) => {
                // handle task error
                error!("Task failed: {}", e);
            }
        }
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
                    grid_color[0].saturating_sub(30),
                    grid_color[1].saturating_sub(30),
                    grid_color[2].saturating_sub(30),
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
            if y % 10 < 5 {
                // Vertical lines 1.5px wide
                image.put_pixel(x, y, image::Rgba(grid_lines_color));
            }
        }
    }
    for y in (0..94).step_by(20) {  // Adjusted to avoid overflowing the height
        for x in 0..440 {
            if x % 10 < 5 {
                // Horizontal lines 1.5px wide
                image.put_pixel(x, y, image::Rgba(grid_lines_color));
            }
        }
    }

    Ok(DynamicImage::ImageRgba8(image))
}

async fn download_and_process(badge_info: BadgeInfo) -> Result<(RgbaImage, String, (i64, i64), (i64, i64)), anyhow::Error> {
    let badge_image = reqwest::get(&badge_info.image_url).await?.bytes().await?;
    let image_format = image::guess_format(&badge_image)?;

    let badge_dynamic_image = match image_format {
        ImageFormat::Gif => {
            debug!("Loading GIF badge {}", badge_info.to_id_string());
            let decoder = GifDecoder::new(&badge_image[..])?;
            let frames = decoder.into_frames().collect_frames()?;
            let middle_frame = frames[frames.len() / 2].clone();
            DynamicImage::from(middle_frame.into_buffer())
        },
        ImageFormat::Png => {
            debug!("Loading PNG badge {}", badge_info.to_id_string());
            image::load_from_memory_with_format(&badge_image, ImageFormat::Png)?
        },
        _ => {
            error!("Unexpected image format ({:?}) for badge {}. Skipping this image.", image_format, badge_info.to_id_string());
            return Err(anyhow::anyhow!("Unexpected image format"));
        }
    }.to_rgba8();

    let (x, y) = badge_info.to_offset_location();
    Ok((badge_dynamic_image, badge_info.to_id_string(), (x, y), (badge_info.xloc, badge_info.yloc)))
}