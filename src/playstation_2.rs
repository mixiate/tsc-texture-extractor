pub fn decode_rgb5(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(width as _, height as _);

    let mut i = 0;
    for y in 0..height {
        for x in 0..width {
            let pixel_index = i * 2;

            let bits = u16::from_le_bytes(bytes[pixel_index..pixel_index + 2].try_into().unwrap());

            let r = ((bits & 0b0000_0000_0001_1111) << 3) as u8;
            let b = (((bits & 0b0111_1100_0000_0000) >> 10) << 3) as u8;
            let g = (((bits & 0b0000_0011_1110_0000) >> 5) << 3) as u8;

            image.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, 255]));
            i += 1;
        }
    }

    image::imageops::flip_vertical(&image)
}

pub fn decode_rgb8(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(width as _, height as _);

    let mut i = 0;
    for y in 0..height {
        for x in 0..width {
            let r = bytes[i];
            let g = bytes[i + 1];
            let b = bytes[i + 2];
            image.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, 255]));
            i += 3;
        }
    }

    image::imageops::flip_vertical(&image)
}

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
