//! Macros for the WASI HTTP Proxy world.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![recursion_limit = "512"]

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let is_async = input.sig.asyncness.is_some();
    let name = &input.sig.ident;
    let body = &input.block;

    if name != "main" {
        let tokens = quote_spanned! { name.span() =>
            compile_error!("only fn main can be tagged with #[wasi_http_server::main]");
        };
        return TokenStream::from(tokens);
    }

    if !is_async {
        let tokens = quote_spanned! { name.span() =>
            compile_error!("fn main must be `async fn main`");
        };
        return TokenStream::from(tokens);
    }

    let inputs = &input.sig.inputs;
    let result = match inputs.len() {
        3 => {
            let (_reactor, _reactor_ty) = arg_to_string(&inputs[0]);
            let (_req, _req_ty) = arg_to_string(&inputs[1]);
            let (_res, _res_ty) = arg_to_string(&inputs[2]);

            quote! {
                struct _Component;

                ::wasi::http::proxy::export!(_Component);

                impl wasi::exports::http::incoming_handler::Guest for _Component {
                    fn handle(#_req: #_req_ty, #_res: #_res_ty) {
                        wasi_async_runtime::block_on(|#_reactor: #_reactor_ty| async move {
                            #body
                        })
                    }
                }
            }
        }
        _ => {
            let tokens = quote_spanned! { inputs.span() =>
                compile_error!("fn main should take 3 arguments");
            };
            return TokenStream::from(tokens);
        }
    };

    result.into()
}

fn arg_to_string(arg: &syn::FnArg) -> (syn::Ident, syn::Type) {
    let pat = match arg {
        syn::FnArg::Typed(pat) => pat,
        _ => panic!("expected an ident as an arg to `fn main`"),
    };
    let ident = match &*pat.pat {
        syn::Pat::Ident(ident) => ident.ident.clone(),
        _ => panic!("expected an ident as an arg to `fn main`"),
    };
    (ident, *pat.ty.clone())
}
