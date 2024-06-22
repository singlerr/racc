#![feature(proc_macro_expand)]
#![feature(let_chains)]
#![feature(extend_one)]

mod def_parser;
mod def_compiler;

use proc_macro::{Span, TokenStream, TokenTree};
use quote::quote;
use crate::def_parser::DefParser;

#[proc_macro]
pub fn lex(input: TokenStream) -> TokenStream{
    let mut input = input.clone();
    let mut def_parser = DefParser::new(&mut input);

    while let Some(def) = def_parser.next_def() {

    }

    let gen = quote!{

    };
    gen.into()
}