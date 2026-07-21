//! GAIA benchmark evaluation pipeline.
//!
//! [GAIA](https://huggingface.co/datasets/gaia-benchmark/GAIA) is a general AI assistant benchmark.
//! This module loads the dataset, asks an LLM to solve each problem with structured output,
//! and evaluates whether the predicted answer matches the ground truth.
//!
//! # Pipeline
//! 1. `dataset::load_gaia_level1` – fetch problems (local cache or HuggingFace API)
//! 2. `solver::solve_problem_with_retry` – ask the LLM, retry on transient failures
//! 3. `evaluator::evaluate_gaia_single` – compare prediction vs. answer
//! 4. `bin/gaia.rs` – orchestrates parallel evaluation across models

pub mod models;
pub mod dataset;
pub mod solver;
pub mod evaluator;