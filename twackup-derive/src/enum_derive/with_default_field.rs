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

use super::utils::enum_field_name;
use crate::enum_derive::utils::get_convert_all_attr;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput};

pub(crate) fn derive(input: &DeriveInput) -> TokenStream {
    let enum_name = &input.ident;

    let data_enum = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("Only enum type is supported"),
    };

    let convert_all_form = get_convert_all_attr(input);
    let convert_all_form = convert_all_form.as_ref();

    let as_str_content = as_str_iterator(convert_all_form, data_enum);
    let from_str_content = from_str_iterator(convert_all_form, data_enum);

    let expanded = quote! {
        #[automatically_derived]
        impl #enum_name {
            #[inline]
            #[doc = "Converts self to static string or exposes internal contents"]
            pub fn as_str(&self) -> &str {
                match self {
                    #(#as_str_content),*
                }
            }
        }

        #[automatically_derived]
        impl From<&str> for #enum_name {
            fn from(string: &str) -> Self {
                match string {
                    #(#from_str_content),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn as_str_iterator<'a>(
    convert_all_form: Option<&'a String>,
    data_enum: &'a DataEnum,
) -> impl Iterator<Item = impl quote::ToTokens> + 'a {
    data_enum.variants.iter().map(move |variant| {
        let name = &variant.ident;
        assert!(
            variant.fields.len() <= 1,
            "Enum variant has more than 1 field"
        );

        let string_name = enum_field_name(variant, convert_all_form);

        if variant.fields.is_empty() {
            quote!(Self::#name => #string_name)
        } else {
            let temp = quote!(val);
            quote!(Self::#name(#temp) => #temp.as_str())
        }
    })
}

fn from_str_iterator<'a>(
    convert_all_form: Option<&'a String>,
    data_enum: &'a DataEnum,
) -> impl Iterator<Item = impl quote::ToTokens> + 'a {
    data_enum.variants.iter().map(move |variant| {
        let name = &variant.ident;
        assert!(
            variant.fields.len() <= 1,
            "Enum variant has more than 1 field"
        );

        if variant.fields.is_empty() {
            let string_name = enum_field_name(variant, convert_all_form);
            quote!(#string_name => Self::#name)
        } else {
            quote!(other => Self::#name(other.into()))
        }
    })
}
