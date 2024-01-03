
use crate::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

#[test]
fn ffi_impl_test() {
    let original = quote! {
        #[cfg(foo)]
        #[cfg(bar)]
        pub fn test<T>(a: i32, mut b: T, &mut c: String) -> String
        where
            T: Into<bool>,
        {
            if a > 3 && b.into() {
                "True".to_owned()
            } else {
                c.to_owned()
            }
        }
    };

    let expected = quote! {
        #[cfg(foo)]
        #[cfg(bar)]
        #[no_mangle]
        pub unsafe extern "C" fn test<T>(a: i32, mut b: T, &mut c: String) -> String
        where
            T: Into<bool>,
        {
            #[allow(unused_mut)]
            unsafe fn call<T>(a: i32, mut b: T, &mut c: String) -> String
            where
                T: Into<bool>,
            {
                if a.is_null() || b.is_null() || c.is_null() {
                    return <String as net_fi::FFIResult>::null_arg_error();
                }

                if a > 3 && b.into() {
                    "True".to_owned()
                } else {
                    c.to_owned()
                }
            }
            <String as net_fi::FFIResult>::catch(move || call(a, b, c))
        }
    };
    
    let expected_parsed: ItemFn = syn::parse2(expected.clone()).unwrap();

    let original: ItemFn = syn::parse2(original).unwrap();

    let actual = ffi_impl(original);
    let actual_parsed: ItemFn = syn::parse2(actual.clone()).unwrap();

    if expected_parsed != actual_parsed {

        println!("Expected value:");
        println!("```");
        print_item(&expected);
        println!("```");

        println!("\nBut was:");
        println!("```");
        print_item(&actual);
        println!("```");

        assert!(false);
    }
}

fn print_item(item: &TokenStream) {
    let item = syn::parse2(item.clone()).unwrap();
    let file = syn::File {
        attrs: vec![],
        items: vec![item],
        shebang: None,
    };

    println!("{}", prettyplease::unparse(&file));
}
