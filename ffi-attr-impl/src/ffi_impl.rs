use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    ItemFn,
    ReturnType,
    visit::Visit,
};

use crate::args_visitor::ArgsVisitor;

pub fn ffi_impl(item: ItemFn) -> TokenStream {
    // Localize all key function elements
    let attrs = item.attrs;
    let vis = item.vis;
    let constness = item.sig.constness;
    let asyncness = item.sig.asyncness;
    let ident = item.sig.ident;
    let generics = item.sig.generics.params;
    let inputs = item.sig.inputs;
    let output = item.sig.output;
    let where_clause = item.sig.generics.where_clause;
    let statements = item.block.stmts;

    // Retrieve all arguments
    let mut args_visitor = ArgsVisitor::new();
    inputs.iter().for_each(|arg| args_visitor.visit_fn_arg(arg));
    let arg_idents = args_visitor.idents;

    // Retrieve return type
    let unit_ty = syn::parse2(quote!{()}).unwrap();
    let return_type = match &output {
        ReturnType::Type(_, ty) => ty,
        _ => &unit_ty,
    };

    let expanded = quote! {
        #(#attrs)
        *
        #[no_mangle]
        #vis #constness #asyncness unsafe extern "C" fn #ident<#generics>(#inputs) #output #where_clause {

            #[allow(unused_mut)]
            unsafe fn call<#generics>(#inputs) #output #where_clause {
                if #(#arg_idents.is_null()) || * {
                    return <#return_type as net_fi::FFIResult>::null_arg_error();
                }

                #(#statements)*
            }
            <#return_type as net_fi::FFIResult>::catch(move || call(#(#arg_idents),*))
        }
    };
    expanded
}