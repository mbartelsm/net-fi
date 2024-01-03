use ffi_attr_impl::ffi_impl;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn ffi(
    _attribute: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(item as ItemFn);
    ffi_impl(parsed).into()
}
