mod domain;
mod board;
mod rules;
mod game;

pub use domain::*;
pub use board::*;
pub use rules::{is_king_checked,is_king_mated};
