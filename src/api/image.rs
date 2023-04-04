use image::{GenericImageView};
use image::imageops::FilterType;

fn get_ascii(intensity: u8) -> &'static str {
    let ascii: [&str; 8] = [" ", ".", ",", "-", "+", "=", "@", "#"];
    return ascii[(intensity/32) as usize]
}

pub async fn request_image(id: i32, size: u32) -> Result<(), reqwest::Error> {
    let endpoint = format!("https://avatars.githubusercontent.com/u/{}", id);
    let image_bytes = reqwest::get(&endpoint).await?
        .bytes().await?;

    let image = image::load_from_memory(&image_bytes).unwrap()
        .resize(size, size, FilterType::Nearest);

    for y in (0..size).step_by(2) {
        let mut row: Vec<&'static str> = Vec::new();
        for x in 0..size {
            let pix = image.get_pixel(x,y);
            let mut intensity = pix[0]/3 + pix[1]/3 + pix[2]/3;
            if pix[3] == 0 {
                intensity = 0;
            }
            row.push(get_ascii(intensity));
        }
        println!("{}", row.join(""));
    }

    Ok(())
}
