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

#[derive(Clone, PartialEq)]
pub enum Priority {
    Optional,
    Required,
    Important,
    Standard,
    Unknown,
}

impl Priority {
    pub fn from_string(value: &str) -> Priority {
        match value.to_lowercase().as_str() {
            "optional" => Priority::Optional,
            "required" => Priority::Required,
            "important" => Priority::Important,
            "standard" => Priority::Standard,
            _ => Priority::Unknown,
        }
    }

    pub fn from_string_opt(value: Option<&String>) -> Priority {
        match value {
            Some(value) => Self::from_string(value),
            None => Priority::Unknown
        }
    }
}
