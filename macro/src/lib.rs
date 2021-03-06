extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::convert::TryFrom;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Expr, ExprField, LitByte, LitStr, Token,
};

fn parse_pubkey_literal(
    input: ParseStream,
    pubkey_type: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
    let pubkey = if input.peek(syn::LitStr) {
        let pubkey_literal: LitStr = input.parse()?;
        decode_pubkey(&pubkey_literal, &pubkey_type)?
    } else {
        let expr: Expr = input.parse()?;
        quote! { #expr }
    };

    if !input.is_empty() {
        let stream: proc_macro2::TokenStream = input.parse()?;
        return Err(syn::Error::new_spanned(stream, "unexpected token"));
    }
    Ok(pubkey)
}

struct KeyDecoder(proc_macro2::TokenStream);

impl Parse for KeyDecoder {
    fn parse(input: ParseStream) -> Result<Self> {
        parse_pubkey_literal(input, quote! { ::solana_program::pubkey::Pubkey }).map(Self)
    }
}

impl ToTokens for KeyDecoder {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let id = &self.0;
        tokens.extend(quote! {#id});
    }
}

fn decode_pubkey(
    id_literal: &LitStr,
    pubkey_type: &proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
    let id_vec = bs58::decode(id_literal.value())
        .into_vec()
        .map_err(|_| syn::Error::new_spanned(&id_literal, "failed to decode base58 string"))?;
    let id_array = <[u8; 32]>::try_from(<&[u8]>::clone(&&id_vec[..])).map_err(|_| {
        syn::Error::new_spanned(
            &id_literal,
            format!("pubkey array is not 32 bytes long: len={}", id_vec.len()),
        )
    })?;
    let bytes = id_array.iter().map(|b| LitByte::new(*b, Span::call_site()));
    Ok(quote! {
        #pubkey_type::new_from_array(
            [#(#bytes,)*]
        )
    })
}

struct EqualityChecker {
    field: ExprField,
    pubkey: KeyDecoder,
}
impl Parse for EqualityChecker {
    fn parse(input: ParseStream) -> Result<Self> {
        let field: ExprField = input.parse()?;
        input.parse::<Token![,]>()?;
        let pubkey: KeyDecoder = input.parse()?;
        Ok(EqualityChecker { field, pubkey })
    }
}
/// Given a struct with a field that is of type solana_program::pubkey::Pubkey, check to see whether or not that pubkey is equal to a specific string literal in base58 encoding.
#[proc_macro]
pub fn assert_pubkeys_equal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let EqualityChecker { field, pubkey } = parse_macro_input!(input as EqualityChecker);
    let local = quote! { #pubkey};
    TokenStream::from(quote! {
        assert_eq!(#field, #local);
    })
}
