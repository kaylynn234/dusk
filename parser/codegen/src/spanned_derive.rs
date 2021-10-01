use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Expr, Item, ItemEnum, ItemStruct, Variant};

pub fn spanned_derive(input: TokenStream) -> TokenStream {
    let (item_ident, tokens) = match parse_macro_input!(input as Item) {
        Item::Enum(item) => (item.ident.clone(), impl_enum(&item)),
        Item::Struct(item) => (item.ident.clone(), impl_struct(&item)),
        _ => panic!(),
    };

    let tokens = quote! {
        impl crate::span::Spanned for #item_ident {
            fn span(&self) -> crate::span::Span {
                #tokens
            }
        }
    };

    tokens.into()
}

fn impl_enum(item: &ItemEnum) -> TokenStream2 {
    let variants = item.variants.iter().map(impl_enum_variant);

    quote! {
        match &self {
            #(#variants),*
        }
    }
}

fn impl_enum_variant(variant: &Variant) -> TokenStream2 {
    let variant_ident = &variant.ident;
    match &variant.fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            quote! {
                Self::#variant_ident(field___) => field___.span()
            }
        }
        _ => panic!("Spanned can only be derived on enums where all variants have one tuple field"),
    }
}

fn impl_struct(item: &ItemStruct) -> TokenStream2 {
    item.attrs
        .iter()
        .find(|attr| attr.path.is_ident("span"))
        .expect("an attribute of the form `#[span(..)]` is required")
        .parse_args::<Expr>()
        .expect("the span attribute must be of the form `#[span(..)]`")
        .to_token_stream()
}
