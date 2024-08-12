mod ls;
mod cat;
mod touch;
mod wget;

use std::{env, fs};
use crate::cat::cat;
use crate::ls::ls;
use crate::touch::touch;
use crate::wget::wget;

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
        },
        "touch" => {
            let file_name = &args[2];
            touch(file_name)
        }
        "wget" => {
            let url = &args[2];
            wget(url)
        },
        _ => println!("No command selected")
    }
}
