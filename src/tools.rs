//! Tool registry for function-calling demos.
//!
//! Each tool is an OpenAI-compatible function definition that the model can invoke.
//! The agent executes the corresponding local Rust code and feeds the result back
//! into the conversation so the model can continue reasoning.

use std::collections::HashMap;

use crate::tools::{
    calculator::r#impl::CalculatorTool, tool::Tool, web_search::r#impl::WebSearchTool,
};

pub mod calculator;
pub mod tool;
pub mod web_search;

pub type ToolBox = HashMap<String, Box<dyn Tool>>;

pub fn build_toolbox() -> ToolBox {
    let tools: Vec<Box<dyn Tool>> = vec![Box::new(CalculatorTool), Box::new(WebSearchTool)];

    tools
        .into_iter()
        .map(|tool| (tool.name().to_string(), tool))
        .collect()
}
