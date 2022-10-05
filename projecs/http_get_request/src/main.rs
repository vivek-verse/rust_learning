#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://wevek.com")
        .await
        .unwrap()
        .text()
        .await;
    println!("{:#?}", resp);
    Ok(())
}
//https://blog.logrocket.com/making-http-requests-rust-reqwest/
