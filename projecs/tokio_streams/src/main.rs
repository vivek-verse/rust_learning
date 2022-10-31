use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream =
        tokio_stream::iter(["My", "name", "is", "Vivek"]).map(|s| s.to_ascii_uppercase());

    while let Some(s) = stream.next().await {
        println!("{s}");
    }
}
