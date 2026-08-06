
use schemars::schema_for;

use crate::tools::{
    tool::Tool,
    web_search::execute::{WebSearchArgs, search_web},
};

pub struct WebSearchTool;

#[async_trait::async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &'static str {
        "web_search"
    }

    fn description(&self) -> &'static str {
        "Search the web for information on a given query."
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schema_for!(WebSearchArgs))
            .expect("Failed to serialize web search arguments")
    }

    async fn execute(&self, args_json: &str) -> anyhow::Result<String> {
        let args: WebSearchArgs = serde_json::from_str(args_json)?;
        let result = search_web(args).await;
        match result {
            Ok(output) => Ok(serde_json::to_string(&output)?),
            Err(e) => Ok(format!("Web search failed: {}", e)),
        }
    }
}
