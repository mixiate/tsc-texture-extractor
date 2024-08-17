pub fn list_textures(datasets_bytes: &[u8], endianness: crate::Endianness) -> Vec<(String, u32, &[u8])> {
    let mut files = Vec::new();

    let identifier = u32::from_le_bytes(*b"TXFL");
    for (position, window) in datasets_bytes.windows(4).enumerate() {
        if endianness.u32_from_bytes(window.try_into().unwrap()) == identifier {
            let file_id = endianness.u32_from_bytes(datasets_bytes[position - 12..position - 8].try_into().unwrap());
            let file_size =
                endianness.u32_from_bytes(datasets_bytes[position - 8..position - 4].try_into().unwrap()) as usize;
            let file_bytes = &datasets_bytes[position..position + file_size];

            let null_position = file_bytes[4..].iter().position(|x| *x == 0).unwrap();
            let name = std::str::from_utf8(&file_bytes[4..4 + null_position]).unwrap();

            files.push((
                name.to_owned(),
                file_id,
                &datasets_bytes[position..position + file_size],
            ));
        }
    }

    files
}
