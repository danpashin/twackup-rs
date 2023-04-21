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

use convert_case::{Case, Casing};
use syn::{punctuated::Punctuated, token::Comma, Attribute, DeriveInput, Expr, Lit, Meta, Variant};

pub(crate) const ATTRIBUTE_ROOT_NAME: &str = "twackup";
pub(crate) const ATTRIBUTE_RENAME: &str = "rename";
pub(crate) const ATTRIBUTE_CONVERT: &str = "convert";
pub(crate) const ATTRIBUTE_CONVERT_ALL: &str = "convert_all";

pub(crate) fn enum_field_name(variant: &Variant, convert_form: Option<&String>) -> String {
    let meta = twackup_attributes_meta(&variant.attrs);
    let name = meta.iter().find_map(|meta| match meta {
        Meta::NameValue(m) if m.path.is_ident(ATTRIBUTE_RENAME) => Some(expr_get_string(&m.value)),
        Meta::NameValue(m) if m.path.is_ident(ATTRIBUTE_CONVERT) => {
            let value = expr_get_string(&m.value);
            let to_return = variant.ident.to_string();
            Some(convert_string(to_return, Some(value.as_str())))
        }
        _ => None,
    });
    name.unwrap_or_else(|| {
        convert_string(variant.ident.to_string(), convert_form.map(String::as_str))
    })
}

pub(crate) fn convert_string(string: String, form: Option<&str>) -> String {
    let converted = match form {
        Some("upper") => string.to_case(Case::Upper),
        Some("lower") => string.to_case(Case::Lower),
        Some("camel") => string.to_case(Case::Camel),
        Some("pascal") => string.to_case(Case::Pascal),
        Some("kebab") => string.to_case(Case::Kebab),
        Some("train") => string.to_case(Case::Train),
        Some("title") => string.to_case(Case::Title),
        Some("snake") => string.to_case(Case::Snake),
        Some(other) => panic!("Unknown convert form {:?}", other),
        None => string,
    };

    converted.split_whitespace().collect()
}

pub(crate) fn twackup_attributes_meta(attributes: &[Attribute]) -> Punctuated<Meta, Comma> {
    attributes
        .iter()
        .flat_map(|attribute| {
            if !attribute.path().is_ident(ATTRIBUTE_ROOT_NAME) {
                return Punctuated::<Meta, Comma>::new();
            }

            attribute
                .parse_args_with(Punctuated::parse_terminated)
                .unwrap_or_default()
        })
        .collect()
}

pub(crate) fn expr_get_string(expr: &Expr) -> String {
    let Expr::Lit(lit_expr) = expr else {
        panic!("Expresssion is not a lit expression!");
    };

    let Lit::Str(string) = &lit_expr.lit else {
        panic!("Value is not a string!");
    };

    string.value()
}

pub(crate) fn get_convert_all_attr(input: &DeriveInput) -> Option<String> {
    let meta = twackup_attributes_meta(&input.attrs);
    meta.iter().find_map(|meta| match meta {
        Meta::NameValue(m) if m.path.is_ident(ATTRIBUTE_CONVERT_ALL) => {
            Some(expr_get_string(&m.value))
        }
        _ => None,
    })
}
