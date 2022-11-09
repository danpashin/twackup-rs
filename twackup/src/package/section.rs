/*
 * Copyright 2020 DanP
 *
 * This file is part of Twackup
 *
 * Twackup is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Twackup is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Twackup. If not, see <http://www.gnu.org/licenses/>.
 */

#[cfg(feature = "cli")]
use ansi_term::Colour;

#[derive(Clone, PartialEq, Eq)]
pub enum Section {
    Other(String),
    Unknown,
    System,
    Tweaks,
    Utilities,
    Packaging,
    Development,
    TerminalSupport,
    Themes,
    Archiving,
    Networking,
    TextEditors,
}

impl Section {
    pub fn from_string(value: &str) -> Section {
        match value.to_lowercase().as_str() {
            "system" => Section::System,
            "tweaks" => Section::Tweaks,
            "utilities" => Section::Utilities,
            "packaging" => Section::Packaging,
            "development" => Section::Development,
            "themes" => Section::Themes,
            "terminal_support" | "terminal support" => Section::TerminalSupport,
            "networking" => Section::Networking,
            "archiving" => Section::Archiving,
            "text_editors" => Section::TextEditors,
            _ => Section::Other(value.to_string()),
        }
    }

    pub fn from_string_opt(value: Option<&String>) -> Section {
        match value {
            Some(value) => Self::from_string(value),
            None => Section::Unknown,
        }
    }

    #[cfg(feature = "cli")]
    pub fn color(&self) -> Colour {
        match self {
            Self::System => Colour::Fixed(9),  // bright red
            Self::Tweaks => Colour::Fixed(11), // bright yellow
            Self::Utilities | Section::Packaging => Colour::Fixed(14), // bright cyan
            Self::Development => Colour::Fixed(130), // more like orange with pink
            Self::Themes => Colour::Fixed(12), // bright blue
            Self::TerminalSupport => Colour::Fixed(10), // bright green
            Self::Networking => Colour::Fixed(112), // bright green with some cyan
            Self::Archiving => Colour::Fixed(216), // peach?
            Self::TextEditors => Colour::Fixed(162), // between red and magenta. Raspberry?
            _ => Colour::Fixed(8),             // bright grey
        }
    }
}
