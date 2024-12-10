mod impls;

use impls::RepeatInput;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Error};

#[proc_macro]
pub fn repeat_for_types(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RepeatInput);

    impls::repeat_for_types(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
