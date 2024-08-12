mod ls;
mod cat;

use std::{env, fs};
use crate::cat::cat;
use crate::ls::ls;

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
