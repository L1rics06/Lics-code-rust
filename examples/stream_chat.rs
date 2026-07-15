
use futures::StreamExt;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use LicRustCode::constant::DEEP_SEEK_V4_FLASH;
use LicRustCode::llm::stream::chat_stream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    // let url = std::env::var("OPENAI_BASE_URL")?;
    //println!("{url}");


    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let s = chat_stream(DEEP_SEEK_V4_FLASH,
                        Some("you are an ai assistant"),
                        "介绍一下rss");

    futures::pin_mut!(s);
    let mut output = String::new();
    while let Some(result) = s.next().await {
        match result {
            Ok(text) => {
                output.push_str(&text);
                print!("{text}");
            }
            Err(err) => {
                eprintln!("Error: {err}");
                break;
            }
        }
    }

    //println!("Result: {output}");
    println!("------------------------------");

    Ok(())
}
