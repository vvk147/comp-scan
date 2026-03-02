// Optional features (Ollama, encryption, scheduler, notifications) are compiled but not yet
// wired into the main flow; allow dead_code/unused_imports until integrated so CI stays green.
#![allow(dead_code, unused_imports)]

pub mod actions;
pub mod ai;
pub mod analyzer;
pub mod cli;
pub mod daemon;
pub mod observer;
pub mod scanner;
pub mod storage;
pub mod ui;
