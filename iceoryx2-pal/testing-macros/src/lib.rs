use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let test_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &test_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_block = &test_fn.block;
    let fn_attrs = &test_fn.attrs;
    let fn_vis = &test_fn.vis;
    let fn_sig = &test_fn.sig;

    let expanded = quote! {
        #[cfg(feature = "std")]
        #(#fn_attrs)*
        #[test]
        #fn_vis #fn_sig #fn_block

        #[cfg(not(feature = "std"))]
        #(#fn_attrs)*
        #fn_vis fn #fn_name() {
            #fn_block
        }

        #[cfg(not(feature = "std"))]
        ::iceoryx2_pal_testing_nostd::inventory::submit! {
            ::iceoryx2_pal_testing_nostd::TestCase {
                name: #fn_name_str,
                test_fn: #fn_name,
            }
        }
    };

    TokenStream::from(expanded)
}
