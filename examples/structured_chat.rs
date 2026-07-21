//! Example: structured output demo.
//!
//! Run with:
//! ```bash
//! cargo run --example structured_chat
//! ```
//!
//! Expects `OPENAI_BASE_URL` and `OPENAI_API_KEY` in `.env`.

use lic_rust_code::constant::{DEEP_SEEK_V4_FLASH, SYSTEM_PROMPT};
use lic_rust_code::llm::structured::chat_complete_structured;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    tracing_subscriber::fmt::init();

    let prompt = "Write a Rust function that reverses a string and includes unit tests.";
    let plan = chat_complete_structured(DEEP_SEEK_V4_FLASH, Some(SYSTEM_PROMPT), prompt)
        .await?;

    println!("{:#?}", plan);
    Ok(())
}
