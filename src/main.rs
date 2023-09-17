use image::{DynamicImage, ImageBuffer , RgbaImage, ImageFormat};
use std::io::{Cursor, Read};
use reqwest;
use std::path::Path;
use tokio;
use clap::{Parser, ArgGroup};

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

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

    // if badges is empty, print a message and return
    if avatar_card.badges.is_empty() {
        println!("No badges found!");
        return Ok(());
    }

    let input_path = Path::new("badges_background.png");
    let output_path = Path::new(&opts.output);

    let mut base_image = tile_image(input_path)?;

    for (_, badge_info) in &avatar_card.badges {
        let badge_image = reqwest::get(&badge_info.image_url).await?.bytes().await?;
        let badge_dynamic_image = image::load_from_memory_with_format(&badge_image, image::ImageFormat::Gif)?.to_rgba8();

        // get the positon from the badge_positions vector
        let item = avatar_card.badge_positions.iter().find(|(id, _, _)| id == &badge_info.to_id_string()).unwrap();
        let x = item.1;
        let y = item.2;

        println!("Overlaying badge {} at ({}, {}) ({}, {})", badge_info.to_id_string(), x, y, badge_info.xloc, badge_info.yloc);

        image::imageops::overlay(&mut base_image, &badge_dynamic_image, x, y);
    }

    base_image.save(output_path)?;

    println!("Image saved successfully!");

    Ok(())
}

fn tile_image(input_path: &Path) -> Result<DynamicImage, image::ImageError> {
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
