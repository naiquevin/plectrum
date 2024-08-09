use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

const VALIDATION_ERR: &'static str = "The 'Plectrum' derive macro only works for non data-bearing enums";

fn enum_variants(data: &Data) -> HashMap<String, String> {
    let mut m = HashMap::new();
    match data {
        Data::Enum(e) => {
            for v in e.variants.iter() {
                let key = format!("{}", v.ident);
                let value = key.clone();
                m.insert(key, value);
                match v.fields {
                    syn::Fields::Unit => { },
                    _ => panic!("{VALIDATION_ERR}"),
                }
            }
        }
        _ => panic!("{VALIDATION_ERR}")
    }
    m
}

fn gen_fn_values(varmap: &HashMap<String, String>) -> TokenStream {
    let mut body = quote! {
        let mut res = std::collections::HashSet::new();
    };
    for val in varmap.values() {
        let val_str = val.as_str();
        body.extend(quote! {
            res.insert(#val_str);
        });
    }

    quote! {
        fn values() -> std::collections::HashSet<&'static str> {
            #body
            res
        }
    }
}

fn gen_method_value(varmap: &HashMap<String, String>) -> TokenStream {
    let mut arms = quote! {};
    for (key, val) in varmap {
        // @TODO: This seems like a workaround. Find out if there's a
        // more straightforward way to achieve this.
        let lhs: TokenStream = format!("Self::{key}").parse().unwrap();
        let rhs = val.as_str();
        arms.extend(quote! {
            #lhs => #rhs,
        });
    }
    quote! {
        fn value(&self) -> &str {
            match self {
                #arms
            }
        }
    }
}

fn gen_fn_from_value(varmap: &HashMap<String, String>) -> TokenStream {
    let mut arms = quote! {};
    for (key, val) in varmap {
        let lhs = val.as_str();
        // @TODO: This seems like a workaround. Find out if there's a
        // more straightforward way to achieve this.
        let rhs: TokenStream = format!("Self::{key}").parse().unwrap();
        arms.extend(quote! {
            #lhs => #rhs,
        })
    }
    quote! {
        fn from_value(s: &str) -> Self {
            match s {
                #arms
                _ => panic!("Unknown value: {s}"),
            }
        }
    }
}

fn gen_trait_impl(ast: DeriveInput) -> TokenStream {
    let DeriveInput { ident, data, .. } = ast;
    let varmap = enum_variants(&data);
    let fn_values = gen_fn_values(&varmap);
    let method_value = gen_method_value(&varmap);
    let fn_from_value = gen_fn_from_value(&varmap);
    quote! {
        #[automatically_derived]
        impl plectrum::Enum for #ident {
            #fn_values

            #method_value

            #fn_from_value
        }
    }
}

#[proc_macro_derive(Plectrum)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input);
    gen_trait_impl(ast).into()
}
