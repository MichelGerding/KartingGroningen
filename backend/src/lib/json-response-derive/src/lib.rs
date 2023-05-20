use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(JsonResponse)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;

    let output = quote! {
        impl<'r> Responder<'r, 'r> for #ident {
            fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'r> {
                let json = serde_json::to_string(&self).unwrap();

                Response::build()
                    .header(ContentType::new("application", "json"))
                    .sized_body(json.len(), std::io::Cursor::new(json))
                    .ok()
            }
        }
    };

    output.into()
}
