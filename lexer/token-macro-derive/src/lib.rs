use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{punctuated::Punctuated, Ident, ItemEnum, Token};

#[proc_macro_derive(TokenInfo, attributes(category, symbol))]
pub fn token_info_derive(input: TokenStream) -> TokenStream {
    let mut category_results = HashMap::new();
    let item: ItemEnum = syn::parse(input).expect("`TokenInfo` can only be derived for enums");

    if item.variants.is_empty() {
        panic!("Cannot derive `TokenInfo` on an empty enum")
    }

    for variant in item.variants {
        let helper_attr = variant
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("category"))
            .map(|attr| -> Punctuated<Ident, Token![,]> {
                attr.parse_args_with(Punctuated::parse_terminated)
                    .expect("Expected one or more identifiers.")
            });

        if let Some(category_names) = helper_attr {
            for category_name in category_names {
                category_results
                    .entry(category_name)
                    .or_insert_with(Vec::new)
                    .push(variant.ident.clone())
            }
        }
    }

    let enum_name = item.ident;
    let mut category_output = Vec::new();

    for (category_name, fields) in category_results {
        let output_rule = quote! {
            (#category_name) => {#(#enum_name::#fields) |*};
        };

        category_output.push(output_rule);
    }

    let lower_name = enum_name.to_string().to_lowercase();
    let category_macro_name = quote::format_ident!("{}_category", lower_name);

    let tokens = quote! {
        #[macro_export]
        macro_rules! #category_macro_name {
            #(#category_output)*
        }
    };

    tokens.into()
}
