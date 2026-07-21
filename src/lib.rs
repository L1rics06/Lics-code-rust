//! LicRustCode - A Rust-based LLM Code Agent
//!
//! This crate provides a learning-oriented implementation of an LLM-powered code agent.
//! It demonstrates multiple OpenAI API interaction patterns (blocking, streaming, structured)
//! and includes a GAIA benchmark evaluation pipeline.
//!
//! # Architecture Overview
//!
//! - `llm`: Core LLM interaction layer (blocking chat, streaming chat, structured output)
//! - `models`: Data structures for structured LLM responses (ActionPlan, etc.)
//! - `constant`: Model identifiers and system prompts
//! - `tools`: External tool definitions (calculator) for function-calling demos
//! - `gaia`: GAIA benchmark dataset loading, problem solving, and evaluation
//!
//! # Quick Start
//!
//! Set `OPENAI_BASE_URL` and `OPENAI_API_KEY` in `.env`, then run:
//!
//! ```bash
//! cargo run --example stream_chat     # Streaming chat demo
//! cargo run --example structured_chat # Structured output demo
//! cargo run --example tool_call_complete # Function calling demo
//! cargo run --bin gaia               # GAIA benchmark evaluation
//! ```

pub mod llm;
pub mod constant;
pub mod models;
pub mod gaia;
pub mod tools;