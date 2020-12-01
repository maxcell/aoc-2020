use reqwest;
use reqwest::header::COOKIE;
use std::env;
use std::fs::{read_to_string, OpenOptions};
use std::io::copy;

#[tokio::main]
pub async fn get_input(day_input: String) -> Result<(), reqwest::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("./day-{}/src/input.txt", day_input))
        .expect("Didn't create the file. Check if one already exists");

    let day_input_url = format!("https://adventofcode.com/2020/day/{}/input", day_input);
    let cookie = env::var("AOC_SESSION").unwrap();
    println!("Requesting input...");
    let response = reqwest::Client::new()
        .get(&day_input_url)
        .header(COOKIE, cookie)
        .send()
        .await?
        .text()
        .await?;
    copy(&mut response.as_bytes(), &mut file).expect("Couldn't copy response to the file");
    println!("Successfully created the input for day {}", day_input);
    Ok(())
}

pub fn read_input(day_input: String) -> Result<String, std::io::Error> {
    let input_path = format!("./day-{}/src/input.txt", day_input);
    let input_contents = read_to_string(&input_path);

    match input_contents {
        Ok(val) => Ok(val),
        Err(_err) => {
            println!("Failed to read; Attempting to create...");
            get_input(day_input.clone()).unwrap();
            read_input(day_input.clone())
        }
    }
}
