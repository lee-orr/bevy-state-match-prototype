#![forbid(missing_docs)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

mod injected_methods;
mod state;
mod state_matching;

pub use injected_methods::*;
pub use state::*;
pub use state_matching::*;
