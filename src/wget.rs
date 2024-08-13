use std::fs::File;
use std::io::Cursor;
use reqwest::blocking::Client;
use std::time::SystemTime;
use chrono::prelude::*;
use reqwest::header::{HeaderMap};

pub fn wget(url: &String) -> Result<(), Box<dyn std::error::Error>> {
    let curr_time = SystemTime::now();
    let dt: DateTime<Utc> = curr_time.clone().into();

    let file_name = String::from(dt.format("%d%b%Y%H%M%S").to_string());

    let client = Client::new();

    let response = client.get(url).send()?;

    let bytes = response.bytes()?;
    let mut reader = Cursor::new(bytes);

    let mut response = client.get(url).send()?;
    let headers = response.headers();
    let content_type = headers.get("content-type");
    println!("{:#?}",content_type);

    if let Some(content_type) = content_type {
        let content_type = content_type.to_str()?;
        if content_type.contains("html") {
            println!("{:?}", file_name.clone() + &String::from(".html"));
            let mut file = File::create(file_name + &String::from(".html"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("zip") {
            println!("{:?}", file_name.clone() + &String::from(".zip"));
            let mut file = File::create(file_name + &String::from(".zip"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("octet-stream") {
            println!("{:?}", file_name.clone() + &String::from(".exe"));
            let mut file = File::create(file_name + &String::from(".exe"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("text/css") {
            println!("{:?}", file_name.clone() + &String::from(".css"));
            let mut file = File::create(file_name + &String::from(".css"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("text/csv") {
            println!("{:?}", file_name.clone() + &String::from(".csv"));
            let mut file = File::create(file_name + &String::from(".csv"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("application/gzip") {
            println!("{:?}", file_name.clone() + &String::from(".gz"));
            let mut file = File::create(file_name + &String::from(".gz"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("image/gif") {
            println!("{:?}", file_name.clone() + &String::from(".gif"));
            let mut file = File::create(file_name + &String::from(".gif"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("application/java-archive") {
            println!("{:?}", file_name.clone() + &String::from(".jar"));
            let mut file = File::create(file_name + &String::from(".jar"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("image/jpeg") {
            println!("{:?}", file_name.clone() + &String::from(".jpg"));
            let mut file = File::create(file_name + &String::from(".jpg"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("text/javascript") {
            println!("{:?}", file_name.clone() + &String::from(".js"));
            let mut file = File::create(file_name + &String::from(".js"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("audio/mpeg") {
            println!("{:?}", file_name.clone() + &String::from(".mp3"));
            let mut file = File::create(file_name + &String::from(".mp3"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("application/video/mp4") {
            println!("{:?}", file_name.clone() + &String::from(".mp4"));
            let mut file = File::create(file_name + &String::from(".mp4"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("video/mpeg") {
            println!("{:?}", file_name.clone() + &String::from(".mpeg"));
            let mut file = File::create(file_name + &String::from(".mpeg"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("image/png") {
            println!("{:?}", file_name.clone() + &String::from(".png"));
            let mut file = File::create(file_name + &String::from(".png"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("application/vnd.ms-powerpoint") {
            println!("{:?}", file_name.clone() + &String::from(".ppt"));
            let mut file = File::create(file_name + &String::from(".ppt"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("application/vnd.rar") {
            println!("{:?}", file_name.clone() + &String::from(".rar"));
            let mut file = File::create(file_name + &String::from(".rar"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("application/x-sh") {
            println!("{:?}", file_name.clone() + &String::from(".sh"));
            let mut file = File::create(file_name + &String::from(".sh"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("image/svg+xml") {
            println!("{:?}", file_name.clone() + &String::from(".svg"));
            let mut file = File::create(file_name + &String::from(".svg"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("application/x-tar") {
            println!("{:?}", file_name.clone() + &String::from(".tar"));
            let mut file = File::create(file_name + &String::from(".tar"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("text/plain") {
            println!("{:?}", file_name.clone() + &String::from(".txt"));
            let mut file = File::create(file_name + &String::from(".txt"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else if content_type.contains("application/x-7z-compressed") {
            println!("{:?}", file_name.clone() + &String::from(".7z"));
            let mut file = File::create(file_name + &String::from(".7z"))?;
            std::io::copy(&mut reader, &mut file)?;
        } else {
            println!("Sorry we don't provide this extension yet...")
        }
    }

    Ok(())
}