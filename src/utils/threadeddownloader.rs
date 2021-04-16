use reqwest::blocking::get;
use std::cmp::min;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
fn filesize(path: &std::path::Path) -> Result<u64, std::io::Error> {
    let x = path.metadata()?.len();
    Ok(x)
}
#[allow(unused_assignments)]
pub fn threadeddownload(url: &str, output: &str) {
    let mut file_size: u64 = 1000;
    let mut file = File::create(output).unwrap();
    let file_path = Path::new(output);
    match get(url.to_string()) {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                file_size = response.content_length().unwrap();
                match response.bytes() {
                    Ok(byte) => {
                        thread::spawn(move || {
                            file.write(&byte);
                        });
                        let mut downloaded = filesize(file_path).unwrap();

                        let pb = ProgressBar::new(file_size);
                        pb.set_style(ProgressStyle::default_bar()
                            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})")
                            .progress_chars("#>-"));

                        while downloaded < file_size {
                            let new = min(downloaded + 223211, file_size);
                            downloaded = new;
                            pb.set_position(new);
                            thread::sleep(Duration::from_millis(12));
                        }

                        pb.finish_with_message("downloaded");
                    }
                    Err(err) => eprintln!("Could Not Read Response JSON, {}", err),
                }
            } else {
                println!("Response Was Not 200 OK");
            }
        }
        Err(err) => eprintln!("Failed To Send Request: {}", err),
    }
    // println!("{}", data);
}
