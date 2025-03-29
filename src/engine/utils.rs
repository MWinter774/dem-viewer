pub fn string_to_cstring(string: &String) -> std::ffi::CString {
    std::ffi::CString::new(&string[..]).unwrap()
}

pub fn get_contents_of_file(file_path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(file_path)
}

pub fn load_texture_image(path: &str) -> (Vec<u8>, usize, usize) {
    // Open and decode the image file
    let img = image::open(path).expect("Failed to open texture image!");

    // Convert the image to RGB8 format.
    let img = img.to_rgb8();

    // Retrieve the dimensions of the image
    let (width, height) = img.dimensions();

    // Convert the image into a raw byte vector
    let data = img.into_raw();

    (data, width as usize, height as usize)
}
