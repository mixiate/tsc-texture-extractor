pub fn convert(bytes: &[u8], multiply_alpha: bool) -> image::RgbaImage {
    let palette = &bytes[1..(1 + (256 * 4))];

    let bytes = &bytes[(256 * 4) + 5..];

    let mut decoded_bytes = std::vec::Vec::new();

    let mut i = 0;
    while i < bytes.len() - 1 {
        let count = bytes[i] as i8;
        i += 1;

        if count.is_positive() {
            for _ in 0..i32::from(count) {
                decoded_bytes.push(bytes[i]);
            }
            i += 1;
        } else {
            let length = usize::from(count.unsigned_abs());
            decoded_bytes.extend_from_slice(&bytes[i..i + length]);
            i += length;
        }
    }

    let mut pixels = Vec::new();

    for palette_index in decoded_bytes {
        pixels.push(palette[usize::from(palette_index) * 4]);
        pixels.push(palette[(usize::from(palette_index) * 4) + 1]);
        pixels.push(palette[(usize::from(palette_index) * 4) + 2]);

        let alpha = palette[(usize::from(palette_index) * 4) + 3];
        let alpha = if multiply_alpha { alpha.saturating_mul(2) } else { alpha };
        pixels.push(alpha);
    }

    let image = image::RgbaImage::from_raw(256, 256, pixels).unwrap();
    image::imageops::flip_vertical(&image)
}
