fn compact(x: usize) -> usize {
    let mut x = x & 0x55555555;
    x = (x ^ (x >> 1)) & 0x33333333;
    x = (x ^ (x >> 2)) & 0x0f0f0f0f;
    x = (x ^ (x >> 4)) & 0x00ff00ff;
    x = (x ^ (x >> 8)) & 0x0000ffff;
    x
}

fn deswizzle_palette(
    bytes: &[u8],
    dimensions: usize,
    output: &mut image::RgbaImage,
    palette: &[u8],
    x_offset: usize,
    y_offset: usize,
) {
    for (pixel_index, palette_index) in bytes.iter().enumerate().take(dimensions * dimensions) {
        let x = compact(pixel_index);
        let y = compact(pixel_index >> 1);

        let palette_index = usize::from(*palette_index);
        let r = palette[palette_index * 4];
        let g = palette[(palette_index * 4) + 1];
        let b = palette[(palette_index * 4) + 2];
        let a = palette[(palette_index * 4) + 3];
        output.put_pixel(
            (x + x_offset) as u32,
            ((dimensions - 1 - y) + y_offset) as u32,
            image::Rgba([r, g, b, a]),
        );
    }
}

pub fn decode_palette(bytes: &[u8], width: usize, height: usize, palette: &[u8]) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(width as _, height as _);

    if width == height {
        deswizzle_palette(bytes, width, &mut image, palette, 0, 0);
    }

    if width > height {
        let section_count = width / height;
        for i in 0..section_count {
            let section_bytes = &bytes[i * (height * height)..];
            deswizzle_palette(section_bytes, height, &mut image, palette, i * height, 0);
        }
    }

    if height > width {
        let section_count = height / width;
        for i in 0..section_count {
            let section_bytes = &bytes[i * (width * width)..];
            let section_height_offset = (section_count * width) - ((i + 1) * width);
            deswizzle_palette(section_bytes, width, &mut image, palette, 0, section_height_offset);
        }
    }

    image
}

fn deswizzle_rgba(bytes: &[u8], dimensions: usize, output: &mut image::RgbaImage, x_offset: usize, y_offset: usize) {
    for i in 0..(dimensions * dimensions) {
        let x = compact(i);
        let y = compact(i >> 1);

        let r = bytes[i * 4];
        let g = bytes[(i * 4) + 1];
        let b = bytes[(i * 4) + 2];
        let a = bytes[(i * 4) + 3];
        output.put_pixel(
            (x + x_offset) as u32,
            ((dimensions - 1 - y) + y_offset) as u32,
            image::Rgba([r, g, b, a]),
        );
    }
}

pub fn decode_rgba(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(width as _, height as _);

    if width == height {
        deswizzle_rgba(bytes, width, &mut image, 0, 0);
    }

    if width > height {
        let section_count = width / height;
        for i in 0..section_count {
            let section_bytes = &bytes[i * (height * height) * 4..];
            deswizzle_rgba(section_bytes, height, &mut image, i * height, 0);
        }
    }

    if height > width {
        let section_count = height / width;
        for i in 0..section_count {
            let section_bytes = &bytes[i * (width * width) * 4..];
            let section_height_offset = (section_count * width) - ((i + 1) * width);
            deswizzle_rgba(section_bytes, width, &mut image, 0, section_height_offset);
        }
    }

    image
}

fn deswizzle_rgb5(bytes: &[u8], dimensions: usize, output: &mut image::RgbaImage, x_offset: usize, y_offset: usize) {
    for i in 0..(dimensions * dimensions) {
        let x = compact(i);
        let y = compact(i >> 1);

        let bits = u16::from_le_bytes(bytes[i * 2..(i * 2) + 2].try_into().unwrap());

        let r = (((bits & 0b0111_1100_0000_0000) >> 10) << 3) as u8;
        let g = (((bits & 0b0000_0011_1110_0000) >> 5) << 3) as u8;
        let b = ((bits & 0b0000_0000_0001_1111) << 3) as u8;

        output.put_pixel(
            (x + x_offset) as u32,
            ((dimensions - 1 - y) + y_offset) as u32,
            image::Rgba([r, g, b, 255]),
        );
    }
}

pub fn decode_rgb5(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(width as _, height as _);

    if width == height {
        deswizzle_rgb5(bytes, width, &mut image, 0, 0);
    }

    if width > height {
        let section_count = width / height;
        for i in 0..section_count {
            let section_bytes = &bytes[i * (height * height) * 2..];
            deswizzle_rgb5(section_bytes, height, &mut image, i * height, 0);
        }
    }

    if height > width {
        let section_count = height / width;
        for i in 0..section_count {
            let section_bytes = &bytes[i * (width * width) * 2..];
            let section_height_offset = (section_count * width) - ((i + 1) * width);
            deswizzle_rgb5(section_bytes, width, &mut image, 0, section_height_offset);
        }
    }

    image
}

pub fn decode_bc1(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let mut decompressed_pixels = vec![0u8; width * height * 4];
    texpresso::Format::Bc1.decompress(bytes, width, height, &mut decompressed_pixels);
    let image = image::RgbaImage::from_raw(width as u32, height as u32, decompressed_pixels).unwrap();
    image::imageops::flip_vertical(&image)
}

pub fn decode_bc2(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let mut decompressed_pixels = vec![0u8; width * height * 4];
    texpresso::Format::Bc2.decompress(bytes, width, height, &mut decompressed_pixels);
    let image = image::RgbaImage::from_raw(width as u32, height as u32, decompressed_pixels).unwrap();
    image::imageops::flip_vertical(&image)
}
