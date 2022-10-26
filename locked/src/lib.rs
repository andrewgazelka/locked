extern crate proc_macro;
use proc_macro::TokenStream;
use sha2::{Digest, Sha256};
use syn::{parenthesized, parse::ParseStream, DeriveInput};

struct LockedAttr {
    hash: String,
}

impl syn::parse::Parse for LockedAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        let hash: syn::LitStr = content.parse()?;
        let hash = hash.value();

        Ok(Self { hash })
    }
}
#[proc_macro_derive(Locked, attributes(locked))]
pub fn derive_locked(item: TokenStream) -> TokenStream {
    let raw_str = item.to_string();
    
    let raw_str: String = raw_str
        .lines().
        filter(|line| !line.starts_with("#[locked"))
        .collect();
    let derive_input = syn::parse_macro_input!(item as DeriveInput);

    let attribute = derive_input
        .attrs
        .iter()
        .find(|a| a.path.is_ident("locked"))
        .expect("need to provide a #[locked({hash}] attribute on struct header");
    
    let tokens = &attribute.tokens;
    
    let LockedAttr {
        hash: expected_hash,
    } = syn::parse2(tokens.clone()).expect("could not parse tokens");

    let mut hasher = Sha256::new();
    hasher.update(raw_str.as_bytes());
    let result = hasher.finalize();

    let got_hash = hex::encode(result);
    
    /// the number of hash characters which will be used
    const HASH_CHARS_NEEDED: usize = 7;
    let got_hash = &got_hash[..HASH_CHARS_NEEDED];

    assert_eq!(expected_hash, got_hash);

    TokenStream::new()
}
