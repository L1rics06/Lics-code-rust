//! Example: function calling (tool use) demo.
//!
//! Run with:
//! ```bash
//! cargo run --example tool_call_complete
//! ```
//!
//! Expects `OPENAI_BASE_URL` and `OPENAI_API_KEY` in `.env`.
//! Registers a calculator tool so the model can perform arithmetic.

use anyhow::Ok;
use lic_rust_code::{constant::DEEP_SEEK_V4_FLASH, llm::complete::chat_complete, tools::tools};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let tools = tools();

    let plan = chat_complete(
        DEEP_SEEK_V4_FLASH,
        Some("你是一个全能的助手"),
        "尼泊尔的首都是哪里？",
        tools.clone(),
    )
    .await?;

    tracing::info!("Response: {plan:#?}");
    println!("-----------------------------------------------");

    let plan = chat_complete(
        DEEP_SEEK_V4_FLASH,
        Some("你是一个全能的助手"),
        "5876乘以675是多少？",
        tools.clone(),
    )
    .await?;

    tracing::info!("Response: {plan:#?}");

    Ok(())
}
