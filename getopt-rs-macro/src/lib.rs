
extern crate syn;

use proc_macro::TokenStream;
use syn::{Expr, parse::Parse, punctuated::Punctuated, token::Comma};
use syn::{parse::ParseStream, Result, parse_macro_input, Error};
use quote::quote;

#[derive(Debug)]
struct GetoptArgs {
    iterator: Expr,
    parsers: Punctuated<Expr, Comma>,
    sets: Punctuated<Expr, Comma>,
}

impl Parse for GetoptArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut parsers = Punctuated::new();
        let mut sets = Punctuated::new();
        let iterator: Expr = input.parse()?;
        let _: Comma = input.parse()?;

        while ! input.is_empty() {
            let parse: Expr = input.parse()?;

            parsers.push_value(parse);
            parsers.push_punct(input.parse()?);

            let set: Expr = input.parse()?;

            sets.push_value(set);
            if input.is_empty() {
                break;
            }
            sets.push_punct(input.parse()?);
        }
        Ok(GetoptArgs {
            iterator,
            parsers,
            sets,
        })
    }
}

#[cfg(not(feature="async"))]
#[proc_macro]
pub fn getopt(input: TokenStream) -> TokenStream {
    let getopt_args = parse_macro_input!(input as GetoptArgs);

    let iterator = match &getopt_args.iterator {
        Expr::Path(path) => {
            quote! {
                &mut #path
            }
        }
        Expr::Reference(reference) => {
            if reference.mutability.is_some() {
                quote! {
                    #reference
                }
            }
            else {
                Error::new_spanned(reference, "need an instance or a mutable reference").to_compile_error()
            }
        }
        expr => {
            quote! { #expr }
        }
    };

    let mut getopt_init = quote! {
        let mut parsers: Vec<Box<dyn Parser>> = vec![];
    };

    getopt_init.extend(
    getopt_args.parsers.iter().zip(getopt_args.sets.iter())
            .map(|(p, s)| {
                quote! { 
                    #s.subscribe_from(&mut #p);
                    #p.publish_to(Box::new(#s));
                    parsers.push(Box::new(#p)); 
                }
            }
    ));

    let ret = quote! {{
        #getopt_init
        getopt_impl(#iterator, parsers)
    }};
    ret.into()
}


#[cfg(feature="async")]
#[proc_macro]
pub fn getopt(input: TokenStream) -> TokenStream {
    let getopt_args = parse_macro_input!(input as GetoptArgs);

    let iterator = match &getopt_args.iterator {
        Expr::Path(path) => {
            quote! {
                &mut #path
            }
        }
        Expr::Reference(reference) => {
            if reference.mutability.is_some() {
                quote! {
                    #reference
                }
            }
            else {
                Error::new_spanned(reference, "need an instance or a mutable reference").to_compile_error()
            }
        }
        expr => {
            quote! { #expr }
        }
    };

    let mut getopt_init = quote! {
        let mut parsers: Vec<Box<dyn Parser>> = vec![];
    };

    getopt_init.extend(
    getopt_args.parsers.iter().zip(getopt_args.sets.iter())
            .map(|(p, s)| {
                quote! { 
                    #s.subscribe_from(&mut #p);
                    #p.publish_to(Box::new(#s));
                    parsers.push(Box::new(#p)); 
                }
            }
    ));

    let ret = quote! {async {
        #getopt_init
        getopt_impl(#iterator, parsers)
    }};
    ret.into()
}