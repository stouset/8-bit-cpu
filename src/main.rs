// Lint Groups
#![warn(deprecated_safe)]
#![warn(future_incompatible)]
#![warn(nonstandard_style)]
#![warn(refining_impl_trait)]
#![warn(rust_2024_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]

// Individual Lints
#![warn(deprecated_in_future)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(non_ascii_idents)]
#![warn(noop_method_call)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

// Lint Extensions
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]
#![warn(rustdoc::all)]

// Lint Exceptions
#![allow(clippy::upper_case_acronyms)]

// TODO: remove
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

pub mod grammar;

use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::{self, Config};
use logos::Logos;
use grammar::{tokenize, Parser, Token};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        std::process::exit(1);
    }
    let source = std::fs::read_to_string(&args[1])?;
    let mut diags = vec![];
    let (tokens, ranges) = tokenize(Token::lexer(&source), &mut diags);
    let cst = Parser::parse(&source, tokens, ranges, &mut diags);
    println!("{cst}");
    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = Config::default();
    let file = SimpleFile::new(&args[1], &source);
    for diag in &diags {
        term::emit(&mut writer.lock(), &config, &file, diag).unwrap();
    }
    Ok(())
}
