use proc_macro::TokenStream;
mod spanned_derive;
mod visitor_derive;

// This is done because of a proc macro limitation. We can't re-export this from a submodule.
#[proc_macro_derive(Visitor, attributes(visit))]
pub fn visitor_derive(input: TokenStream) -> TokenStream {
    visitor_derive::visitor_derive(input)
}

#[proc_macro_derive(Spanned, attributes(span))]
pub fn spanned_derive(input: TokenStream) -> TokenStream {
    spanned_derive::spanned_derive(input)
}
