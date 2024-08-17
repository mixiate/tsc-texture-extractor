pub fn decode_cmpr(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let decompressed_data = gctex::decode(bytes, width as _, height as _, gctex::TextureFormat::CMPR, &[], 0);
    let image = image::RgbaImage::from_raw(width as _, height as _, decompressed_data.to_vec()).unwrap();
    image::imageops::flip_vertical(&image)
}

pub fn decode_rgb5a3(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let decompressed_data = gctex::decode(bytes, width as _, height as _, gctex::TextureFormat::RGB5A3, &[], 0);
    let image = image::RgbaImage::from_raw(width as _, height as _, decompressed_data.to_vec()).unwrap();
    image::imageops::flip_vertical(&image)
}

pub fn decode_rgba8(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let decompressed_data = gctex::decode(bytes, width as _, height as _, gctex::TextureFormat::RGBA8, &[], 0);

    let image = image::RgbaImage::from_raw(width as _, height as _, decompressed_data.to_vec()).unwrap();
    image::imageops::flip_vertical(&image)
}

pub fn decode_c4(bytes: &[u8], width: usize, height: usize, palette: &[u8]) -> image::RgbaImage {
    let decompressed_data = gctex::decode(bytes, width as _, height as _, gctex::TextureFormat::I4, &[], 0);

    let mut deswizzled_palette = Vec::new();
    for i in 0..16 {
        deswizzled_palette.push(palette[(i * 2) + 1]);
        deswizzled_palette.push(palette[i * 2]);
        deswizzled_palette.push(palette[(i * 2) + 32 + 1]);
        deswizzled_palette.push(palette[(i * 2) + 32]);
    }

    let mut pixels = Vec::new();
    for pixel in decompressed_data[0..].iter().step_by(4) {
        pixels.push(deswizzled_palette[(usize::from(*pixel) / 16) * 4]);
        pixels.push(deswizzled_palette[(usize::from(*pixel) / 16) * 4 + 1]);
        pixels.push(deswizzled_palette[(usize::from(*pixel) / 16) * 4 + 2]);
        pixels.push(deswizzled_palette[(usize::from(*pixel) / 16) * 4 + 3]);
    }

    let image = image::RgbaImage::from_raw(width as _, height as _, pixels).unwrap();
    image::imageops::flip_vertical(&image)
}

pub fn decode_c8(bytes: &[u8], width: usize, height: usize, palette: &[u8]) -> image::RgbaImage {
    let mut decompressed_data = gctex::decode(bytes, width as _, height as _, gctex::TextureFormat::I8, &[], 1);

    for a in decompressed_data[3..].iter_mut().step_by(4) {
        *a = 255;
    }

    let mut deswizzled_palette = Vec::new();
    for i in 0..256 {
        deswizzled_palette.push(palette[(i * 2) + 1]);
        deswizzled_palette.push(palette[i * 2]);
        deswizzled_palette.push(palette[(i * 2) + 512 + 1]);
        deswizzled_palette.push(palette[(i * 2) + 512]);
    }

    let mut pixels = Vec::new();
    for pixel in decompressed_data[0..].iter().step_by(4) {
        pixels.push(deswizzled_palette[usize::from(*pixel) * 4]);
        pixels.push(deswizzled_palette[usize::from(*pixel) * 4 + 1]);
        pixels.push(deswizzled_palette[usize::from(*pixel) * 4 + 2]);
        pixels.push(deswizzled_palette[usize::from(*pixel) * 4 + 3]);
    }

    let image = image::RgbaImage::from_raw(width as _, height as _, pixels).unwrap();
    image::imageops::flip_vertical(&image)
}
