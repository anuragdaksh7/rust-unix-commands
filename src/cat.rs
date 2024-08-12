use std::fs;

pub fn cat(path: &String) {
    let contents = fs::read_to_string(path)
        .expect("Couldn't read file");
    println!("{contents}");
}
