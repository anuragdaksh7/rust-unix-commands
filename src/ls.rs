use std::fs;

pub fn ls(path: &String) {
    let dir = String::from(path);

    println!("{dir}");
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}
