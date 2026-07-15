
use anyhow::Ok;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use LicRustCode::constant::DEEP_SEEK_V4_FLASH;
use LicRustCode::llm::complete::chat_complete;
use LicRustCode::llm::stream::chat_stream;
use LicRustCode::llm::structured::chat_complete_structured;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    // let url = std::env::var("OPENAI_BASE_URL")?;
    //println!("{url}");


    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let plan = chat_stream(DEEP_SEEK_V4_FLASH
                                        ,Some("you are an ai assistant")
                                        ,"介绍一下rss").await?;

    futrues


    Ok(())
}
