//! Repository Module
//!
//! Abstraction layer for process instance persistence.
//!
//! Currently provides in-memory implementation, designed for future database integration.

pub mod memory;
pub mod traits;

pub use memory::MemoryRepository;
pub use traits::{Repository, RepositoryError};

