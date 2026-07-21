//! Tool registry for function-calling demos.
//!
//! Each tool is an OpenAI-compatible function definition that the model can invoke.
//! The agent executes the corresponding local Rust code and feeds the result back
//! into the conversation so the model can continue reasoning.

use async_openai::types::chat::ChatCompletionTools;

use crate::tools::calculator::definition::calculator_tool_definition;

pub mod calculator;

/// Returns the list of tools available to the LLM.
///
/// Currently only contains the calculator, but additional tools can be added here.
pub fn tools() -> Vec<ChatCompletionTools> {
    vec![calculator_tool_definition()]
}