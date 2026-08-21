//! Tool registry for function-calling demos.
//!
//! Each tool is an OpenAI-compatible function definition that the model can invoke.
//! The agent executes the corresponding local Rust code and feeds the result back
//! into the conversation so the model can continue reasoning.

use std::{collections::HashMap, sync::Arc};

use crate::tools::{
    calculator::r#impl::CalculatorTool,
    mcp::{client::McpClient, tool::McpTool},
    tool::Tool,
    web_search::r#impl::WebSearchTool,
};

pub mod calculator;
pub mod mcp;
pub mod tool;
pub mod web_search;

pub type ToolBox = HashMap<String, Box<dyn Tool>>;

pub async fn build_toolbox() -> anyhow::Result<ToolBox> {
    let mut tools: Vec<Box<dyn Tool>> = vec![Box::new(CalculatorTool), Box::new(WebSearchTool)];


    //我们定义的Tool trait 实际上是贴合 rmcp的定义的tool的
    let mcp_client = Arc::new(McpClient::connect().await?);
    for tool in mcp_client.list_tools().await? {
        tools.push(Box::new(McpTool::new(mcp_client.clone(), tool)));
    }

    Ok(tools
        .into_iter()
        .map(|tool| (tool.name().to_string(), tool))
        .collect())
}
