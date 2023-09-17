use image::{DynamicImage, ImageBuffer , RgbaImage};
use reqwest;
use std::path::Path;
use tokio;
use clap::Parser;

mod models;
mod api;

use api::fetch_avatar_profile_card;


#[derive(Parser)]
struct Opts {
    #[arg(short, long, help = "The user ID")]
    cid: i64,

    #[arg(short, long, default_value = "canvas.png", help = "The output file name")]
    output: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let avatar_card = fetch_avatar_profile_card(opts.cid).await?;

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

        let x = badge_info.xloc;
        let y = badge_info.yloc - 200;

        println!("Overlaying badge {} at ({}, {})", badge_info.to_id_string(), x, y);

        image::imageops::overlay(&mut base_image, &badge_dynamic_image, x, y);
    }

    base_image.save(output_path)?;

    println!("Image saved successfully!");

    Ok(())
}

fn tile_image(input_path: &Path) -> Result<DynamicImage, image::ImageError> {
    let img = image::open(input_path)?.to_rgba8();
    let mut output_img: RgbaImage = ImageBuffer::new(440, 100);
    for x in (0..440).step_by(40) {
        for y in (0..100).step_by(40) {
            image::imageops::replace(&mut output_img, &img, x, y);
        }
    }
    Ok(DynamicImage::ImageRgba8(output_img))
}
