use anyhow::Context;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WebSearchArgs {
    pub query: String,

    #[schemars(range(min = 0, max = 20))]
    #[serde(default = "default_max_results")]
    pub max_results: u8,

    #[serde(default = "default_topic")]
    pub topic: String,

    #[serde(default)]
    pub time_range: Option<String>,
}

fn default_max_results() -> u8 {
    5
}

fn default_topic() -> String {
    "general".to_string()
}

#[derive(Debug, Deserialize,Serialize)]
struct TavilyRequest<'a> {
    api_key: &'a str,
    query: &'a str,
    max_results: u8,
    topic: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_range: Option<&'a str>,

    search_depth: &'a str,
    include_answer: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct TavilyResponse {
    pub results: Vec<SearchResult>,
    answer: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WebSearchOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    pub results: Vec<SearchResult>,
}

pub async fn search_web(args: WebSearchArgs) -> anyhow::Result<WebSearchOutput> {
    let api_key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY not set in .env file");

    let body = TavilyRequest {
        api_key: &api_key,
        query: &args.query,
        max_results: args.max_results,
        topic: &args.topic,
        time_range: args.time_range.as_deref(),
        search_depth: "advanced",
        include_answer: true,
    };

    let resp = reqwest::Client::new()
        .post("https://api.tavily.com/search")
        .json(&body)
        .send()
        .await
        .context("Failed to send request to Tavily API")?;

    if !resp.status().is_success() {
        anyhow::bail!("Tavily API request failed with status {}", resp.status());
    }

    let parsed: TavilyResponse = resp
        .json()
        .await
        .context("Failed to parse Tavily API response")?;

    Ok(WebSearchOutput {
        answer: parsed.answer,
        results: parsed.results,
    })
}
