//! Calculator - A simple calculator application for CachyOS (KDE Plasma).
//!
//! Built with Rust and the iced GUI framework.

pub mod app;
pub mod calculator;

fn main() -> iced::Result {
    app::run()
}