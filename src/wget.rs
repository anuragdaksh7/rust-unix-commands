use std::fs::File;
use reqwest::blocking::Client;
use std::time::SystemTime;
use chrono::prelude::*;
use log::debug;

pub fn wget(url: &String) {
    let curr_time = SystemTime::now();
    let dt: DateTime<Utc> = curr_time.clone().into();

    let file_name = String::from(dt.format("%d-%b-%Y-%H:%M:%S-%P-%z").to_string());

    let client = Client::new();

    let mut response = client.get(url).send();

    println!("{:?}", response.unwrap().text())
}