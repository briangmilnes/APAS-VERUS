// Copyright (c) 2025 Brian G. Milnes
//! Per veracity/docs/Accepted.md: #[accepted_external_body] expands to #[verifier::external_body].
//! Veracity sees the source attribute and can treat as info.

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn accepted_external_body(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let out = quote! {
        #[verifier::external_body]
        #item
    };
    TokenStream::from(out)
}
