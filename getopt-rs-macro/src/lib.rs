
extern crate syn;

use proc_macro::TokenStream;
use syn::{Expr, parse::Parse, punctuated::Punctuated, token::Comma};
use syn::{parse::ParseStream, Result, parse_macro_input, Error};
use quote::quote;

#[derive(Debug)]
struct GetoptArgs {
    iterator: Option<Expr>,
    parsers: Punctuated<Expr, Comma>,
    sets: Punctuated<Expr, Comma>,
}

enum ParseState {
    PSParser,
    PSSet,
}

impl Parse for GetoptArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut parsers = Punctuated::new();
        let mut sets = Punctuated::new();
        let mut state = ParseState::PSParser;
        let iterator: Expr = input.parse()?;
        let comma: Comma = input.parse()?;

        while ! input.is_empty() {
            match state {
                ParseState::PSParser => {
                    let parse: Expr = input.parse()?;
                    parsers.push(parse);
                    state = ParseState::PSSet;
                    if input.is_empty() {
                        break;
                    }
                    parsers.push_punct(input.parse()?);
                }
                ParseState::PSSet => {
                    let set: Expr = input.parse()?;
                    sets.push(set);
                    state = ParseState::PSParser;
                    if input.is_empty() {
                        break;
                    }
                    sets.push_punct(input.parse()?);
                }
            }
        }

        if parsers.len() > sets.len() {
            sets.push_value(iterator);
            sets.push_punct(comma);
            Ok(GetoptArgs {
                iterator: None,
                parsers: sets,
                sets: parsers,
            })
        }
        else {
            Ok(GetoptArgs {
                iterator: Some(iterator),
                parsers,
                sets,
            })
        }
    }
}

#[cfg(not(feature="async"))]
#[proc_macro]
pub fn getopt(input: TokenStream) -> TokenStream {
    let getopt_args = parse_macro_input!(input as GetoptArgs);

    let iterator = match getopt_args.iterator.as_ref() {
        Some(iterator) => {
            match iterator {
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
            }
        }
        None => {
            quote! {
                &mut ai
            }
        }
    };

    let mut getopt_init = match getopt_args.iterator.as_ref() {
        Some(_) => {
            quote! {
                let mut parsers: Vec<Box<dyn Parser<DefaultSet, DefaultIdGen>>> = vec![];
            }
        }
        None => {
            quote! {
                let mut parsers: Vec<Box<dyn Parser<DefaultSet, DefaultIdGen>>> = vec![];
                let mut ai = ArgIterator::new();
                
                ai.set_args(&mut std::env::args().skip(1));
            }
        }
    };

    getopt_init.extend(
    getopt_args.parsers.iter().zip(getopt_args.sets.iter())
            .map(|(p, s)| {
                quote! { 
                    #s.subscribe_from(&mut #p);
                    #p.publish_to(#s);
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

    let iterator = match getopt_args.iterator.as_ref() {
        Some(iterator) => {
            match iterator {
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
            }
        }
        None => {
            quote! {
                &mut ai
            }
        }
    };

    let mut getopt_init = match getopt_args.iterator.as_ref() {
        Some(_) => {
            quote! {
                let mut parsers: Vec<Box<dyn Parser<DefaultSet, DefaultIdGen>>> = vec![];
            }
        }
        None => {
            quote! {
                let mut parsers: Vec<Box<dyn Parser<DefaultSet, DefaultIdGen>>> = vec![];
                let mut ai = ArgIterator::new();
                
                ai.set_args(&mut std::env::args().skip(1));
            }
        }
    };

    getopt_init.extend(
    getopt_args.parsers.iter().zip(getopt_args.sets.iter())
            .map(|(p, s)| {
                quote! { 
                    #s.subscribe_from(&mut #p);
                    #p.publish_to(#s);
                    parsers.push(Box::new(#p)); 
                }
            }
    ));

    let ret = quote! {async {
        #getopt_init
        getopt_impl(#iterator, parsers).await
    }};
    ret.into()
}