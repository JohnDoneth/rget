
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;

use std::str::FromStr;

use tokio::fs::File;
use tokio::prelude::*;

#[tokio::main]
async fn main() {

    match std::env::args().nth(1) {
        Some(ref url) => {

            let url: Url = Url::from_str(url).unwrap();

            let file_name = url.path_segments().unwrap().last().unwrap();
            
            let mut file = File::create(file_name).await.unwrap();
            
            let mut response = reqwest::get(url.clone())
                .await.unwrap();

            let pb = ProgressBar::new(0);

            pb.set_message(&format!("Fetching {}", url));
            
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}) (ETA {eta})")
                .progress_chars("#>-"));


            if let Some(length) = response.content_length() {
                pb.set_length(length);
            }

            while let Some(chunk) = response.chunk().await.unwrap() {
                pb.inc(chunk.len() as u64);

                file.write(&chunk).await.unwrap();
            }

            pb.finish();

        }
        None => {
            println!("Please supply a URL")
        }
    }

    
}
