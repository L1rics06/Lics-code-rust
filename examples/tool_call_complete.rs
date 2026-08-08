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
use lic_rust_code::{
    constant::DEEP_SEEK_V4_FLASH,
    llm::complete::chat_complete,
    tools::{ToolBox, build_toolbox},
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let toolbox = tools();

    let result = chat_complete(
        DEEP_SEEK_V4_FLASH,
        Some("你是一个全能的助手"),
        "今日重庆天气如何？",
        &toolbox,
    )
    .await?;

    tracing::info!("Response: {result:#?}");
    println!("-----------------------------------------------");

    Ok(())
}

fn tools() -> ToolBox {
    // Build and return the default toolbox (e.g., calculator tool)
    build_toolbox()
}
