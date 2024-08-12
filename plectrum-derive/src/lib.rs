use std::collections::HashMap;

use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

use crate::transform::CaseTransform;

mod transform;

const VALIDATION_ERR: &str = "The 'Plectrum' derive macro only works for non data-bearing enums";

fn enum_variants(data: &Data, transform: Option<&CaseTransform>) -> HashMap<String, String> {
    let mut m = HashMap::new();
    match data {
        Data::Enum(e) => {
            for v in e.variants.iter() {
                let key = v.ident.to_string();
                let value = transform.map_or(key.clone(), |t| t.convert(&key));
                m.insert(key, value);
                match v.fields {
                    syn::Fields::Unit => {}
                    _ => panic!("{VALIDATION_ERR}"),
                }
            }
        }
        _ => panic!("{VALIDATION_ERR}"),
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
        // @NOTE: Use of `unwrap` seems acceptable here. The error
        // type is `proc_macro2::LexError` but since we're
        // constructing the String using an enum variant token, we can
        // be fairly sure that it will be ok.
        let lhs: TokenStream = format!("Self::{key}").parse().unwrap();
        let rhs = val;
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
        let lhs = val;
        // @NOTE: Use of `unwrap` seems acceptable here. The error
        // type is `proc_macro2::LexError` but since we're
        // constructing the String using an enum variant token, we can
        // be fairly sure that it will be ok.
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

#[derive(FromDeriveInput)]
#[darling(attributes(plectrum))]
struct Opts {
    rename_all: Option<String>,
}

fn gen_trait_impl(ast: DeriveInput) -> TokenStream {
    let opts = Opts::from_derive_input(&ast).expect("Wrong options");
    let DeriveInput { ident, data, .. } = ast;
    let transform = opts.rename_all.map(CaseTransform::from);
    let varmap = enum_variants(&data, transform.as_ref());
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

#[proc_macro_derive(Plectrum, attributes(plectrum))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input);
    gen_trait_impl(ast).into()
}
