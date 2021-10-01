use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, Item, Token, TypePath};

syn::custom_keyword!(base);
syn::custom_keyword!(node);

enum Node {
    Visit(TypePath),
    Base,
}

fn parse_node_type_args(input: syn::parse::ParseStream) -> syn::Result<TypePath> {
    input.parse::<node>()?;
    input.parse::<Token![=]>()?;
    input.parse()
}

pub fn visitor_derive(input: TokenStream) -> TokenStream {
    let item: Item = syn::parse(input).expect("`Visitor` can only be derived for structs or enums");

    let attr = match &item {
        Item::Enum(enum_) => enum_.attrs.clone(),
        Item::Struct(struct_) => struct_.attrs.clone(),
        _ => panic!("`Visitor` can only be derived for structs or enums"),
    }
    .into_iter()
    .find(|attr| attr.path.is_ident("visit"))
    .expect("a `#[visit(node = ...)]` or `#[visit(base)]` attribute is required");

    let node = attr
        .parse_args_with(parse_node_type_args)
        .map(Node::Visit)
        .or_else(|_| attr.parse_args::<base>().map(|_| Node::Base))
        .expect("invalid syntax - an attribute of the form `#[visit(node = Type)]` or `#[visit(base)]` is required");

    match node {
        Node::Visit(path) => impl_visited(&item, path),
        Node::Base => impl_base(&item),
    }
}

fn impl_base(item: &Item) -> TokenStream {
    let (type_name, type_generics) = match item {
        Item::Enum(enum_) => (&enum_.ident, &enum_.generics),
        Item::Struct(struct_) => (&struct_.ident, &struct_.generics),
        _ => panic!("`Visitor` can only be derived for structs or enums"),
    };

    let tokens = quote! {
        impl#type_generics Visitor<#type_name#type_generics> for #type_name#type_generics {
            fn visit<F>(&self, op: &mut F)
            where
                F: FnMut(&#type_name#type_generics),
            {
                op(self)
            }
        }
    };

    tokens.into()
}

fn impl_visited(item: &Item, type_path: TypePath) -> TokenStream {
    let item = match item {
        Item::Struct(struct_) => struct_,
        _ => panic!("only structs may be visited in this way"),
    };

    let visit_calls = item.fields.iter().enumerate().filter_map(|(index, field)| {
        field
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("visit"))
            .map(|_| {
                field
                    .ident
                    .clone()
                    .unwrap_or_else(|| Ident::new(&index.to_string(), Span::call_site()))
            })
    });

    let type_name = &item.ident;
    let type_generics = &item.generics;
    let type_generic_bounds = item.generics.params.iter();

    let tokens = quote! {
        impl#type_generics Visitor<#type_path> for #type_name#type_generics
        where
            #(#type_generic_bounds: Visitor<#type_path>),*
        {
            fn visit<F>(&self, op: &mut F)
            where
                F: FnMut(&#type_path),
            {
                #(self.#visit_calls.visit(op);)*
            }
        }
    };

    tokens.into()
}
