#![allow(
	clippy::if_same_then_else,
	clippy::len_without_is_empty,
	clippy::module_inception,
	clippy::option_map_unit_fn,
	clippy::unit_arg
)]

mod clipboard;
pub mod completion;
mod context;
pub mod files;
pub mod help;
mod highlighter;
pub mod input;
pub mod manager;
pub mod preview;
pub mod select;
mod step;
pub mod tab;
pub mod tasks;
pub mod which;

pub use clipboard::*;
pub use context::*;
pub use highlighter::*;
pub use step::*;

pub fn init() {
	CLIPBOARD.with(Default::default);

	yazi_scheduler::init();
}
