use std::path::Path;

use crate::gaia::models::{GaiaRow, HfResponse};

const LOCAL_DATA_PATH: &str = "data/gaia_level1.json";
const HF_API_URL: &str = "https://datasets-server.huggingface.co/rows";

pub async fn load_gaia_level1() -> anyhow::Result<Vec<GaiaRow>> {
    // 1. Try local file first so users can bypass gated HF access.
    let local = Path::new(LOCAL_DATA_PATH);
    if local.exists() {
        let text = tokio::fs::read_to_string(local).await?;
        let rows: Vec<GaiaRow> = serde_json::from_str(&text)?;
        return Ok(rows);
    }

    // 2. Fallback to HuggingFace API.
    let token = std::env::var("HF_TOKEN")
        .map_err(|_| anyhow::anyhow!("HF_TOKEN not set. Either set HF_TOKEN (and accept the dataset gate at https://huggingface.co/datasets/gaia-benchmark/GAIA) or place a local file at {LOCAL_DATA_PATH}"))?;

    let client = reqwest::Client::new();
    let http_resp = client
        .get(HF_API_URL)
        .query(&[
            ("dataset", "gaia-benchmark/GAIA"),
            ("config", "2023_level1"),
            ("split", "validation"),
            ("offset", "0"),
            ("length", "10"),
        ])
        .bearer_auth(&token)
        .send()
        .await?;

    let body_text = http_resp.text().await?;
    let parsed: HfResponse = serde_json::from_str(&body_text)
        .map_err(|e| anyhow::anyhow!("Failed to parse HF response: {e}. Body: {body_text}"))?;

    Ok(parsed.rows.into_iter().map(|r| r.row).collect())
}
