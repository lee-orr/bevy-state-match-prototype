#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

mod app;
mod state;
mod state_matching;

pub use app::*;
pub use state::*;
pub use state_matching::*;
