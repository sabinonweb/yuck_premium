use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use std::fs::File;
use reqwest::StatusCode;
use std::str::FromStr;

#[derive(Debug)]
struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer: u32,
}

impl PartialRangeIter {
    fn new(start: u64, end: u64, buffer: u32) -> Result<Self, String> {
        if buffer == 0 {
            Err(format!("Buffer must be greater than 0"))
        } else {
            Ok(PartialRangeIter {
                start,
                end,
                buffer,
            })
        }
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let prev_start = self.start;
            self.start += std::cmp::min(self.buffer as u64, self.end - self.start + 1);
            Some(HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1)).expect("Expected &str, found String"))
        }
    }
}

fn main() -> Result<(), String> {
    let url = "https://httpbin.org/range/102400?duration=2";
    const CHUNK_SIZE: u32 = 10240;

    let client = reqwest::blocking::Client::new();
    let response = client.head(url).send().unwrap();
    let length = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("Response doesn't include the CONTENT_LENGTH field")?;
    let length = u64::from_str(length.to_str().unwrap()).map_err(|_| "Invalid content-length header").unwrap();

    let mut output_file = File::create("download.bin").unwrap();
    
    println!("Starting download....");

    for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE).unwrap() {
        println!("range: {:?}",range);

        let mut response = client.get(url).header(RANGE, range).send().unwrap();

        let status = response.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            return Err(format!("Unexpected server response: {:?}", status));
        }
        std::io::copy(&mut response, &mut output_file).unwrap();
    }

    let mut content = response.text().unwrap();
    std::io::copy(&mut content.as_bytes(), &mut output_file).unwrap();

    println!("Finished with success");

    Ok(())
}
