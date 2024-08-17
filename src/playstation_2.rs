pub fn decode_rgba8(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(width as _, height as _);

    let mut i = 0;
    for y in 0..height {
        for x in 0..width {
            let pixel_index = i * 4;
            let r = bytes[pixel_index];
            let g = bytes[pixel_index + 1];
            let b = bytes[pixel_index + 2];
            let a = bytes[pixel_index + 3];
            image.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a.saturating_mul(2)]));
            i += 1;
        }
    }

    image::imageops::flip_vertical(&image)
}

pub fn decode_c4(bytes: &[u8], width: usize, height: usize, palette: &[u8]) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(width as _, height as _);

    let mut i = 0;
    for y in 0..height {
        for x in 0..width / 2 {
            let palette_index = usize::from(bytes[i] & 0b0000_1111) * 4;
            let r = palette[palette_index];
            let g = palette[palette_index + 1];
            let b = palette[palette_index + 2];
            let a = palette[palette_index + 3];
            image.put_pixel(x as u32 * 2, y as u32, image::Rgba([r, g, b, a.saturating_mul(2)]));

            let palette_index = usize::from(bytes[i] >> 4) * 4;
            let r = palette[palette_index];
            let g = palette[palette_index + 1];
            let b = palette[palette_index + 2];
            let a = palette[palette_index + 3];
            image.put_pixel(x as u32 * 2 + 1, y as u32, image::Rgba([r, g, b, a.saturating_mul(2)]));
            i += 1;
        }
    }

    image::imageops::flip_vertical(&image)
}

pub fn decode_c8(bytes: &[u8], width: usize, height: usize, palette: &[u8]) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(width as _, height as _);

    let mut i = 0;
    for y in 0..height {
        for x in 0..width {
            let palette_index = usize::from(bytes[i]) * 4;
            let r = palette[palette_index];
            let g = palette[palette_index + 1];
            let b = palette[palette_index + 2];
            let a = palette[palette_index + 3];
            image.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a.saturating_mul(2)]));
            i += 1;
        }
    }

    image::imageops::flip_vertical(&image)
}
