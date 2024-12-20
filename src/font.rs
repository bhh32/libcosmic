// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Select preferred fonts.

pub use iced::Font;
use iced_core::font::Weight;

pub fn default() -> Font {
    Font::from(crate::config::interface_font())
}

pub fn light() -> Font {
    Font {
        weight: Weight::Light,
        ..default()
    }
}

pub fn semibold() -> Font {
    Font {
        weight: Weight::Semibold,
        ..default()
    }
}

pub fn bold() -> Font {
    Font {
        weight: Weight::Bold,
        ..default()
    }
}

pub fn mono() -> Font {
    Font::from(crate::config::monospace_font())
}
