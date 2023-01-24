use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HasId)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;

    let output = quote! {
        impl HasIdTrait for #ident {
            fn get_id(&self) -> i32 {
                self.id
            }
        }
    };
    output.into()
}
