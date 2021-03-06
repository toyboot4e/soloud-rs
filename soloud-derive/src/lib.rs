#![recursion_limit = "256"]
#![allow(non_snake_case)]
#![allow(unused_variables)]

extern crate proc_macro;
extern crate syn;

mod audio;
mod filter;
mod load;

use crate::audio::impl_audio_trait;
use crate::filter::impl_filter_trait;
use crate::filter::impl_filter_type_trait;
use crate::load::impl_load_trait;

use proc_macro::TokenStream;

#[proc_macro_derive(AudioExt)]
pub fn audio_source_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_audio_trait(&ast)
}

#[proc_macro_derive(LoadExt)]
pub fn load_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_load_trait(&ast)
}

#[proc_macro_derive(FilterExt)]
pub fn filter_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_filter_trait(&ast)
}

#[proc_macro_derive(FilterAttr)]
pub fn filter_type_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_filter_type_trait(&ast)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
