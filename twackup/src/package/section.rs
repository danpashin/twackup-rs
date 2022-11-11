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
use console::{Color, Style};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Section {
    Archiving,
    Development,
    Networking,
    Packaging,
    System,
    TerminalSupport,
    TextEditors,
    Themes,
    Tweaks,
    Utilities,
    Other(String),
}

impl Section {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Archiving => "Archiving",
            Self::Development => "Development",
            Self::Networking => "Networking",
            Self::Packaging => "Packaging",
            Self::System => "System",
            Self::TerminalSupport => "Terminal Support",
            Self::TextEditors => "Text Editors",
            Self::Themes => "Themes",
            Self::Tweaks => "Tweaks",
            Self::Utilities => "Utilities",
            Self::Other(section) => section.as_str(),
        }
    }

    #[cfg(feature = "cli")]
    #[must_use]
    pub fn color(&self) -> Style {
        let color = match self {
            Self::Archiving => Color::Color256(216),      // peach?
            Self::Development => Color::Color256(130),    // more like orange with pink
            Self::Networking => Color::Color256(112),     // bright green with some cyan
            Self::System => Color::Color256(9),           // bright red
            Self::TerminalSupport => Color::Color256(10), // bright green
            Self::TextEditors => Color::Color256(162),    // between red and magenta. Raspberry?
            Self::Themes => Color::Color256(12),          // bright blue
            Self::Tweaks => Color::Color256(11),          // bright yellow
            Self::Utilities | Self::Packaging => Color::Color256(14), // bright cyan
            Self::Other(_) => Color::Color256(8),         // bright grey
        };

        Style::new().fg(color)
    }
}

impl From<&str> for Section {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "archiving" => Self::Archiving,
            "development" => Self::Development,
            "networking" => Self::Networking,
            "packaging" => Self::Packaging,
            "system" => Self::System,
            "terminal_support" | "terminal support" => Self::TerminalSupport,
            "text_editors" => Self::TextEditors,
            "themes" => Self::Themes,
            "tweaks" => Self::Tweaks,
            "utilities" => Self::Utilities,
            _ => Self::Other(value.to_string()),
        }
    }
}
