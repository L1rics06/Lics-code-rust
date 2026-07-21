//! Structured LLM output via JSON Schema.
//!
//! Forces the model to emit valid JSON matching the [`ActionPlan`] schema,
//! then deserializes it directly into a typed Rust struct.

use async_openai::types::chat::{ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema};
use crate::models::action_plan::ActionPlan;

/// Request an [`ActionPlan`] from the LLM.
///
/// The JSON Schema is generated at runtime by `schemars::schema_for!`
/// and sent to the provider via `response_format`. The provider guarantees
/// (best-effort) that the response is syntactically valid JSON matching that schema.
pub async fn chat_complete_structured(model:&str,
                                       system:Option<&str>,
                                       prompt:&str) -> anyhow::Result<ActionPlan> {
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

    let schema = schemars::schema_for!(ActionPlan);
    let schema_json = schema.as_value().clone();
    let format_setting = ResponseFormat::JsonSchema {
        json_schema:ResponseFormatJsonSchema{
            description:Some("A step-by-step agent plan with difficulty and time estimate"
                .into()),
            name:"action_plan".into(),
            schema:schema_json,
            strict:Some(true),
        },
    };


    let  request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(message)
        .response_format(format_setting)
        .max_tokens(2048u32)
        .build()?;


    let response = client.chat().create(request).await?;

    //tracing::info!("{:#?}",response);
    let plan:ActionPlan = response
        .choices
        .into_iter()
        .next()
        .and_then(|t| t.message.content )
        .ok_or_else(|| anyhow::anyhow!("no chat response received"))
        .and_then(|s| serde_json::from_str::<ActionPlan>(&s).map_err(Into::into))?;

        Ok(plan)
}