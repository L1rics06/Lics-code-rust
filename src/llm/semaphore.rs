//! Global concurrency limiter.
//!
//! Many cloud LLM providers throttle aggressive callers. A semaphore with
//! 3 slots caps the number of in-flight requests across the whole process.

use std::sync::OnceLock;

use tokio::sync::Semaphore;

static SEMAPHORE: OnceLock<Semaphore> = OnceLock::new();

/// Returns a process-wide semaphore initialized to 3 permits.
pub fn get_semaphore() -> &'static Semaphore {
    SEMAPHORE.get_or_init(|| Semaphore::new(3))
}