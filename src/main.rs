use std::{env, fs};

fn ls(path: &String) {
    let dir = String::from(path);

    println!("{dir}");
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}

fn cat(path: &String) {
    let contents = fs::read_to_string(path)
        .expect("Couldn't read file");
    println!("{contents}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "ls" => {
            let path = &args[2];
            ls(path)
        },
        "cat" => {
            let path = &args[2];
            cat(path)
        }
        _ => println!("No command selected")
    }
}
