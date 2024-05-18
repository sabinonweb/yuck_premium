use serde::Deserialize;
use serde_json::json;
use std::env;
use reqwest::{Client, Result};

#[derive(Deserialize, Debug)]
struct Gist {
    id: String,
    html_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let gh_user = env::var("GH_USER").unwrap();
    let gh_pass = env::var("GH_PASS").unwrap();

    let gist_body = json!({
        "description": "the description for this gist",
        "public": true,
        "files": {
             "main.rs": {
             "content": r#"fn main() { println!("hello world!");}"#
            }
        }});

    let request_url = "https://api.github.com/gists";
    let response = Client::new()
        .post(request_url)
        .basic_auth(&gh_user, Some(&gh_pass))
        .json(&gist_body)
        .send().await?;

    let gist: Gist = response.json().await?;
    println!("Created gist {:?}", gist);
    
    let req_url = format!("{}/{}", request_url, gist.id);
    let res = Client::new()
        .delete(req_url)
        .basic_auth(&gh_user, Some(&gh_pass))
        .send().await?;

    println!("Gist deleted {:?}, statuscode {}", gist, res.status());


    Ok(())
}
