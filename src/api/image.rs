use colored::Colorize;
use image::{DynamicImage, GenericImageView, Rgba};
use image::imageops::FilterType;
use anyhow::Result;
use crate::api::request::request;

pub struct ImageData {
    image: DynamicImage
}

impl ImageData {
    pub async fn new(id: i32, size: u32) -> Result<ImageData> {
        let url = &format!("https://avatars.githubusercontent.com/u/{}", id);
        let image_bytes = request(url)
            .await?
            .bytes()
            .await?;

        let image: DynamicImage = image::load_from_memory(&image_bytes)?
            .resize(size*2, size*2, FilterType::Nearest);
        return Ok ( ImageData { image } )
    }

    pub fn average_color(&self) -> (u8, u8, u8) {
        let mut rgb = (0, 0, 0);
        let mut amount = 0;

        for pixel in self.image.pixels() {
            let (_, _, Rgba([r, g, b, a])) = pixel;

            if !(r >= 240 && g >= 240 && b >= 240) && !(r <= 20 && g <= 40 && b <= 20) && (a != 0) {
                amount += 1;
                rgb = (rgb.0 + u32::from(r), rgb.1 + u32::from(g), rgb.2 + u32::from(b));
            }
        }
        return if amount > 0 {
            let average_rgb = ((rgb.0 / amount) as u8, (rgb.1 / amount) as u8, (rgb.2 / amount) as u8);
            average_rgb
        } else {
            (255, 255, 255)
        }
    }

    pub fn get_ascii_art(&self) -> Result<Vec<String>> {
        let (height, width) = self.image.dimensions();

        let mut rows: Vec<String> = Vec::new();
        for y in (0..height).step_by(2) {
            let mut row: Vec<String> = Vec::new();
            for x in 0..width {
                let pixel = self.image.get_pixel(x,y);
                let mut intensity = pixel[0]/3 + pixel[1]/3 + pixel[2]/3;
                if pixel[3] == 0 {
                    intensity = 0;
                }
                let ascii = get_ascii(intensity).truecolor(pixel[0], pixel[1], pixel[2]);
                row.push(ascii.to_string());
            }
            let _ = rows.truncate(height as usize);
            rows.insert(rows.len(), row.join(""));
        }

        Ok(rows)
    }
}

fn get_ascii(intensity: u8) -> &'static str {
    let ascii: [&str; 8] = [" ", "-", "+", "=", "%", "&", "@", "#"];
    return ascii[(intensity / 32) as usize]
}
