pub fn string_to_cstring(string: &String) -> std::ffi::CString {
    std::ffi::CString::new(&string[..]).unwrap()
}

pub fn get_contents_of_file(file_path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(file_path)
}
