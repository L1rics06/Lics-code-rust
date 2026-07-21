//! LLM interaction layer.
//!
//! Provides three chat patterns against the OpenAI-compatible API:
//! - `complete`: blocking request/response with optional tool-calling
//! - `stream`: streaming response with retry logic
//! - `structured`: JSON-schema-constrained response deserialized into Rust structs
//!
//! `semaphore` rate-limits concurrent requests to avoid provider throttling.

pub mod complete;
pub mod structured;
pub mod stream;
pub mod semaphore;