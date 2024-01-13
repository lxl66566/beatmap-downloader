use anyhow::Result;
use std::collections::HashMap;
use tokio;
#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("cmd", "beatmaplist");
    map.insert("type", "hot");
    map.insert("limit", "20");
    let res = client
        .post("https://api.sayobot.cn/?post")
        .json(&map)
        .send()
        .await?
        .json()
        .await?;
    println!("{:?}", res);
    Ok(())
}
