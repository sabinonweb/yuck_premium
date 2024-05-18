use std::fs::File;
use std::io::Read;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), String> {
    let paste_api = "https://paste.rs";
    let mut file = File::open("/Users/sabinonweb/Documents/Projects/downloader/src/message.txt").unwrap();
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // println!("contents {:?}", contents);

    let response = Client::new()
        .post(paste_api)
        .body(contents)
        .send()
        .await
        .unwrap();

    let response_text = response.text().await.unwrap();
    println!("{:?}", response_text);  
    println!("content length :{:?}", response.content_length());

    Ok(())
}
