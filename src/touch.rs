use std::fs;

pub fn touch(file_name: &String) {
    let file = String::from(file_name);

    fs::File::create(file).unwrap();
}