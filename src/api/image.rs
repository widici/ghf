use colored::Colorize;
use image::{DynamicImage, GenericImageView};
use image::imageops::FilterType;

pub struct ImageData {
    image: DynamicImage
}

impl ImageData {
    pub async fn new(id: i32) -> Result<ImageData, reqwest::Error> {
        let endpoint = format!("https://avatars.githubusercontent.com/u/{}", id);
        let image_bytes = reqwest::get(&endpoint).await.unwrap()
            .bytes().await.unwrap();

        let image = image::load_from_memory(&image_bytes).unwrap();
        return Ok( ImageData { image } )
    }

    pub fn get_ascii_art(&self, size: u32) -> Result<Vec<String>, reqwest::Error> {
        let image = self.image.resize(size*2, size*2, FilterType::Nearest);

        let (height, width) = image.dimensions();

        let mut rows: Vec<String> = Vec::new();
        for y in (0..height).step_by(2) {
            let mut row: Vec<String> = Vec::new();
            for x in 0..width {
                let pixel = image.get_pixel(x,y);
                let mut intensity = pixel[0]/3 + pixel[1]/3 + pixel[2]/3;
                if pixel[3] == 0 {
                    intensity = 0;
                }
                let ascii = get_ascii(intensity).truecolor(pixel[0], pixel[1], pixel[2]);
                row.push(ascii.to_string());
            }
            rows.insert(rows.len(), row.join(""));
        }

        Ok(rows)
    }
}

fn get_ascii(intensity: u8) -> &'static str {
    let ascii: [&str; 8] = [" ", "-", "+", "=", "%", "&", "@", "#"];
    return ascii[(intensity / 32) as usize]
}