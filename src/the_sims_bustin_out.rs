pub fn extract_rle_textures(rletextures_path: &std::path::Path, output_path: &std::path::Path) {
    std::fs::create_dir_all(output_path).unwrap();

    let rletextures = std::fs::read(rletextures_path).unwrap();

    let files = crate::arc::read(&rletextures);

    for (name, bytes) in files {
        let image = crate::rle_textures::convert(bytes, true);
        crate::save_texture(image, &name, output_path, false);
    }
}
