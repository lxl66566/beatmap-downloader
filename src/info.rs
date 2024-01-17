use anyhow::Result;

use crate::core::api::API;
pub async fn info(api: &API) -> Result<()> {
    let client = reqwest::Client::new();
    dbg!(&api);
    let res = client
        .post("https://api.sayobot.cn/?post")
        .json(api)
        .send()
        .await?;

    if res.status().is_success() {
        let json_body: serde_json::Value = res.json().await?;
        println!("{:?}", json_body);
    } else {
        println!("Request failed with status code: {:?}", res.status());
    }
    Ok(())
}
