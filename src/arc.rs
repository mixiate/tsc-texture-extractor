pub fn list_files(arc_bytes: &[u8], endianness: crate::Endianness) -> Vec<(String, u32, &[u8])> {
    let list_address = u32::from_le_bytes(arc_bytes[0..4].try_into().unwrap()) as usize;

    let file_count = endianness.u32_from_bytes(arc_bytes[list_address..list_address + 4].try_into().unwrap());

    let mut files = Vec::new();

    let file_list = &arc_bytes[list_address..];
    let mut i = 4;
    for _ in 0..file_count {
        let id = endianness.u32_from_bytes(file_list[i..i + 4].try_into().unwrap());
        i += 4;
        let address = endianness.u32_from_bytes(file_list[i..i + 4].try_into().unwrap()) as usize;
        i += 4;
        let size = endianness.u32_from_bytes(file_list[i..i + 4].try_into().unwrap()) as usize;
        i += 4;

        let null_position = file_list[i..].iter().position(|x| *x == 0).unwrap();
        let name = std::str::from_utf8(&file_list[i..i + null_position]).unwrap();

        files.push((name.to_owned(), id, &arc_bytes[address..address + size]));

        i += null_position;
        i += 9;
    }

    files
}
