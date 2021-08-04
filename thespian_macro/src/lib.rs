use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemTrait};
use quote::quote;

#[proc_macro_attribute]
pub fn thespian(args: TokenStream, input: TokenStream) -> TokenStream {
    let _parsed_args = parse_macro_input!(args as AttributeArgs);
    let parsed_input = parse_macro_input!(input as ItemTrait);

    let output = quote! {
        #parsed_input
    };

    output.into()
}
