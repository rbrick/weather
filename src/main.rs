use std::{error::Error, collections::HashMap};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
    .await?.json::<HashMap<String, String>>().await?;
println!("{:#?}", resp);
Ok(())
}
