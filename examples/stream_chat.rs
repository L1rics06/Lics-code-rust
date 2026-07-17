
use std::result;

use LicRustCode::llm::semaphore::get_semaphore;

use tokio::task::JoinSet;
use tracing::{Instrument, Level};
use tracing_subscriber::FmtSubscriber;
use LicRustCode::constant::{DEEP_SEEK_V4_FLASH, SYSTEM_PROMPT};
use LicRustCode::llm::stream::chat_stream_with_retry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    // let url = std::env::var("OPENAI_BASE_URL")?;
    //println!("{url}");


    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    //there are some prompts to test.
    let prompts = vec![
        "Write a poem about the beauty of nature.",
        "Explain the concept of quantum mechanics in simple terms.",
        "What are the benefits of meditation for mental health?",
        "Describe the process of photosynthesis in plants.",
        "What are the key differences between classical and modern art?",
        "Explain the theory of relativity in layman's terms.",
        "What are the main causes and effects of climate change?",
        "Describe the history and significance of the Great Wall of China.",
        "What are the benefits and drawbacks of social media on society?",
        "Explain the concept of artificial intelligence and its applications."
    ];

    let mut set = JoinSet::new();
    for prompt in prompts {
        let span = tracing::info_span!("chat",prompt=%prompt);
        set.spawn(
            async move {
                tracing::info!("\n\n{prompt}");
                let permit = get_semaphore().acquire().await?;
                let output =
                    chat_stream_with_retry(DEEP_SEEK_V4_FLASH, Some(SYSTEM_PROMPT), prompt)
                    .await?;
                drop(permit);
                Ok::<_, anyhow::Error>((prompt, output))
            }
            .instrument(span)
        );
    }
    //println!("Result: {output}");
    println!("------------------------------");
    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok((prompt, output))) => {
                println!("Prompt: {prompt}\nOutput: {output}\n");
            }
            Ok(Err(err)) => {
                tracing::error!("Error in task: {:?}", err);
            }
            Err(err) => {
                tracing::error!("Task panicked: {:?}", err);
            }
        }
    }

    Ok(())
}

