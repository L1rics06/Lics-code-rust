//! Data models for the GAIA benchmark pipeline.
//!
//! `GaiaRow` represents one problem from the dataset.
//! `GaiaOutput` is the structured response we force the LLM to produce.
//! `GaiaEvalResult` stores the outcome of a single evaluation run.

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Wrapper for the HuggingFace Datasets Server API response.
#[derive(Deserialize, Debug)]
pub struct HfResponse {
    pub rows: Vec<HfRow>,
}

/// Single row wrapper inside [`HfResponse`].
#[derive(Deserialize, Debug)]
pub struct HfRow {
    pub row: GaiaRow,
}

/// One GAIA problem.
///
/// Field names are renamed because the raw dataset uses PascalCase / spaced keys.
#[derive(Deserialize, Debug, Clone)]
pub struct GaiaRow {
    #[serde(rename = "task_id")]
    pub id: String,
    #[serde(rename = "Question")]
    pub question: String,
    #[serde(rename = "Level")]
    pub level: String,
    #[serde(rename = "Final answer")]
    pub final_answer: String,
}

/// Structured output the LLM must emit for every GAIA problem.
///
/// `deny_unknown_fields` ensures the model does not hallucinate extra keys.
#[derive(Deserialize, Debug, JsonSchema, Serialize)]
#[schemars(deny_unknown_fields)]
pub struct GaiaOutput {
    pub is_solvable: bool,
    pub unsolvable_reason: String,
    pub final_answer: String,
}

/// Result of evaluating one problem against one model.
#[derive(Serialize, Debug)]
pub struct GaiaEvalResult {
    pub task_id: String,
    pub model: String,
    pub correct: bool,
    pub is_solvable: Option<bool>,
    pub prediction: Option<String>,
    pub answer: String,
    pub unsolvable_reason: Option<String>,
    pub error: Option<String>,
}