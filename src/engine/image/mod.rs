use image::GenericImageView;

pub struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub rgba_data: Vec<u8>,
}

pub fn decode_image(data: &[u8]) -> Option<DecodedImage> {
    let img = image::load_from_memory(data).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    Some(DecodedImage {
        width,
        height,
        rgba_data: rgba.into_raw(),
    })
}

pub fn get_image_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    let img = image::load_from_memory(data).ok()?;
    Some(img.dimensions())
}