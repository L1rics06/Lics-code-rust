use std::{f32::consts::E, result};
use backon::{ExponentialBuilder, Retryable};

use async_openai::types::chat::{ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use async_stream::stream;use futures::{Stream, StreamExt};
fn chat_stream(model:&str,
                           system:Option<&str>,
                           prompt:&str) -> impl Stream<Item = anyhow::Result<String>> {
    stream! {
        let client =  async_openai::Client::new();
        let mut message = vec![];

        if let Some(system) = system {
            message.push(ChatCompletionRequestSystemMessageArgs::default()
                .content(system)
                .build()?
                .into()
            );
        }

        message.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into()
        );

        let  request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(message)
            .max_tokens(2048u32)
            .build()?;


        let mut stream = client.chat().create_stream(request).await?;

        while let Some(response_result) = stream.next().await {
            match response_result {
                Ok(chunk) => {
                    if let Some(choice) = chunk.choices.first()
                        && let Some(new_text) = &choice.delta.content{
                            yield Ok(new_text.clone())
                    }
                }
                Err(err) => yield Err(err.into())
            }
        }
    }

}

pub async fn chat_stream_with_retry(model:&str,
                           system:Option<&str>,
                           prompt:&str,) -> anyhow::Result<String> {
        let op = || async {
            let s = chat_stream(model, system, prompt);
            futures::pin_mut!(s);
            let mut output = String::new();
            while let Some(result) = s.next().await {
                match result {
                    Ok(text) => {
                        output.push_str(&text);
                        print!("{}", text);
                    }
                    Err(err) => {
                        tracing::error!("Error in chat stream: {:?}", err);
                        return Err(err);
                    }
                }
            }
            Ok(output)
    };
 
    op.retry(ExponentialBuilder::default().with_max_times(3)).await
}
