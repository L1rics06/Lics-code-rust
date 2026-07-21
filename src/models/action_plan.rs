//! Data structures for structured LLM responses.
//!
//! These types are used with `schemars::schema_for!` to generate a JSON Schema
//! that constrains the LLM output to a predictable shape.

use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// A structured plan produced by the LLM.
///
/// The model decomposes a user request into a goal, ordered steps,
/// a difficulty estimate, and a time estimate.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ActionPlan {
    pub goal: String,
    pub action: Vec<ActionStep>,
    pub difficulty: Difficulty,
    pub estimated_minutes: u32,
}

/// A single step inside an [`ActionPlan`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ActionStep {
    pub index: u32,
    pub description: String,
    /// Optional hint about which tool (e.g. `file_read`, `bash_command`) to use.
    pub tool_hint: Option<String>,
}

/// Difficulty level chosen by the model.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}