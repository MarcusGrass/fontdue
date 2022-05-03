//! Fontdue is a font parser, rasterizer, and layout tool.
//!
//! This is a no_std crate, but still requires the alloc crate.

//#![no_std]
#![allow(dead_code)]
#![allow(clippy::style)]
#![allow(clippy::complexity)]

extern crate alloc;

mod font;
mod hash;
/// Tools for laying out strings of text.
pub mod layout;
mod math;
mod platform;
mod raster;
mod table;
mod unicode;

pub use crate::font::*;

/// Alias for Result<T, &'static str>.
pub type FontResult<T> = Result<T, &'static str>;

#[cfg(test)]
mod tests {
    use crate::{Font, FontSettings};

    #[test]
    fn rasterize_oneshot() {
        let jetbrains_mono = "/usr/share/fonts/TTF/JetBrains Mono Medium Nerd Font Complete.ttf";
        let font_bytes = std::fs::read(jetbrains_mono).unwrap();
        let rasterized = Font::render_all(font_bytes.as_slice(), 12.0, FontSettings {
            collection_index: 0,
            scale: 600.0
        }).unwrap();
        eprintln!("{}", rasterized.len());
        drop(rasterized);
        drop(font_bytes);

        loop {

        }
    }
}
