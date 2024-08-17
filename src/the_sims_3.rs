fn decode_rle(bytes: &[u8], bit_count: u8) -> Vec<u8> {
    let rle_bytes_length = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
    let rle_bytes = &bytes[4..4 + rle_bytes_length as usize];

    let mut decompressed_data = Vec::new();

    let mut i = 0;
    while i < rle_bytes.len() - 1 {
        let count = rle_bytes[i];

        for _ in 0..i32::from(count) {
            match bit_count {
                8 => {
                    decompressed_data.push(rle_bytes[i + 1]);
                }
                16 => {
                    decompressed_data.push(rle_bytes[i + 1]);
                    decompressed_data.push(rle_bytes[i + 2]);
                }
                32 => {
                    decompressed_data.push(rle_bytes[i + 1]);
                    decompressed_data.push(rle_bytes[i + 2]);
                    decompressed_data.push(rle_bytes[i + 3]);
                    decompressed_data.push(rle_bytes[i + 4]);
                }
                _ => panic!(),
            }
        }

        i += 1 + usize::from(bit_count) / 8;
    }

    decompressed_data
}

fn convert(bytes: &[u8]) -> image::RgbaImage {
    let bytes = &bytes[16..];
    let null_position = bytes.iter().position(|x| *x == 0).unwrap();

    let width = usize::from(u16::from_be_bytes(
        bytes[null_position + 21..null_position + 23].try_into().unwrap(),
    ));
    let height = usize::from(u16::from_be_bytes(
        bytes[null_position + 23..null_position + 25].try_into().unwrap(),
    ));

    let texture_type = bytes[null_position + 29];

    let texture_bit_count = bytes[null_position + 31];

    let palette_count = usize::from(u16::from_be_bytes(
        bytes[null_position + 25..null_position + 27].try_into().unwrap(),
    ));
    let palette_bit_count = usize::from(bytes[null_position + 32]);

    let palette_length = (palette_count * palette_bit_count) / 8;

    match texture_type {
        0x1 => {
            let bytes = &bytes[null_position + 37..];

            let mut pixels = Vec::new();
            for i in 0..width * height {
                pixels.push(bytes[i * 3]);
                pixels.push(bytes[(i * 3) + 1]);
                pixels.push(bytes[(i * 3) + 2]);
                pixels.push(255);
            }

            let image = image::RgbaImage::from_raw(width as _, height as _, pixels).unwrap();
            image::imageops::flip_vertical(&image)
        }
        0x81 => crate::gamecube::decode_cmpr(&bytes[null_position + 37..], width, height),
        0x82 => {
            let decompressed_data = decode_rle(&bytes[null_position + 37..], texture_bit_count);
            crate::gamecube::decode_rgb5a3(&decompressed_data, width, height)
        }
        0x85 => {
            let decompressed_data = decode_rle(&bytes[null_position + 37..], texture_bit_count);
            crate::gamecube::decode_rgba8(&decompressed_data, width, height)
        }
        0x89 => {
            assert!(palette_bit_count == 32 && palette_count == 16);

            let palette = &bytes[bytes.len() - palette_length..];

            crate::gamecube::decode_c4(&bytes[null_position + 37..], width, height, palette)
        }
        0x8a => {
            assert!(palette_bit_count == 32 && palette_count == 256);

            let decompressed_data = decode_rle(&bytes[null_position + 37..], texture_bit_count);

            let palette = &bytes[bytes.len() - palette_length..];

            crate::gamecube::decode_c8(&decompressed_data, width, height, palette)
        }
        _ => panic!(),
    }
}

pub fn extract_textures(textures_path: &std::path::Path, output_path: &std::path::Path) {
    std::fs::create_dir_all(output_path).unwrap();

    let textures = std::fs::read(textures_path).unwrap();

    let files = crate::arc::read_be(&textures);

    for (name, _, bytes) in files {
        let image = convert(bytes);
        crate::save_texture(image, &name, output_path, SPECULAR_FILE_NAMES.contains(&name.as_str()));
    }
}

const SPECULAR_FILE_NAMES: [&str; 15] = [
    "appliance_bar",
    "count_blanc_bathroom_counter_top",
    "counter_butcherblock",
    "counter_commercial_top",
    "counter_industrial",
    "counter_slat_board",
    "counter_tiled",
    "electronics_tv_plasma_1x3",
    "GL_Doors04",
    "GL_DoorsHandle01",
    "GL_DoorsHandle02",
    "seating_sofa_modern_green",
    "tiled_counter_top",
    "treepear_bark",
    "windsor_door",
];
