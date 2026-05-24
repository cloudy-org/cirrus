#![doc = include_str!("../../README.md")]

pub mod keys;
pub mod error;

#[cfg(feature = "egui")]
pub mod egui;

mod tiny_lexer;