use image::{DynamicImage, ImageBuffer , RgbaImage, ImageFormat};
use std::io::Cursor;
use reqwest;
use std::path::Path;
use tokio;
use clap::{Parser, ArgGroup};
use env_logger;
use log::{debug, info, warn};

mod models;
mod api;

use api::fetch_avatar_profile_card;

const BADGES_BACKGROUND: &[u8] = include_bytes!("badges_background.png");

#[derive(Parser)]
#[clap(group = ArgGroup::new("ArgGroup").required(true).multiple(false))]
struct Opts {
    #[arg(short, long, help = "The user ID", group = "ArgGroup")]
    cid: Option<i64>,

    #[clap(short = 'a', long, group = "ArgGroup", help = "The avatar name")]
    avatar_name: Option<String>,

    #[arg(short, long, default_value = "canvas.png", help = "The output file name")]
    output: String,

    #[clap(short, long, help = "Enable verbose logging")]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let opts: Opts = Opts::parse();

    if opts.verbose {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    } else {
        std::env::set_var("RUST_LOG", "info");
        env_logger::init();
    }

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

    let output_path = Path::new(&opts.output);

    let image_format = match output_path.extension() {
        Some(ext) if ext == "png" => ImageFormat::Png,
        Some(ext) if ext == "jpg" || ext == "jpeg" => ImageFormat::Jpeg,
        _ => {
            log::error!("Unsupported output file format. Please use either .png or .jpg/.jpeg.");
            return Ok(());
        }
    };

    let mut base_image = tile_image()?;

    for (_, badge_info) in &avatar_card.badges {
        let badge_image = reqwest::get(&badge_info.image_url).await?.bytes().await?;
        let badge_dynamic_image = image::load_from_memory_with_format(&badge_image, image::ImageFormat::Gif)?.to_rgba8();

        let x = badge_info.xloc;
        let y = if badge_info.yloc >= 200 {
            warn!("Badge {} is off the bottom of the canvas, moving up", badge_info.to_id_string());
            badge_info.yloc - 200
        } else {
            badge_info.yloc
        };

        debug!("Overlaying badge {} at ({}, {}) ({}, {})", badge_info.to_id_string(), x, y, badge_info.xloc, badge_info.yloc);

        image::imageops::overlay(&mut base_image, &badge_dynamic_image, x, y);
    }

    base_image.save_with_format(output_path, image_format)?;

    info!("Saved image to {}", output_path.display());

    Ok(())
}

fn tile_image() -> Result<DynamicImage, image::ImageError> {
    let cursor = Cursor::new(BADGES_BACKGROUND);
    let img = image::load(cursor, ImageFormat::Png)?;
    let mut output_img: RgbaImage = ImageBuffer::new(440, 100);
    for x in (0..440).step_by(40) {
        for y in (0..100).step_by(40) {
            image::imageops::replace(&mut output_img, &img, x, y);
        }
    }
    Ok(DynamicImage::ImageRgba8(output_img))
}
