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

use ::std::path::Path;

use ::codespan_reporting::diagnostic::Severity;
use ::codespan_reporting::files::SimpleFile;
use ::codespan_reporting::term::{self, Config};
use ::codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use ::color_eyre::{eyre::eyre, Report, Result};
use ::logos::Logos;
use ::tracing_subscriber::EnvFilter;

use crate::grammar::{tokenize, Cst, Parser, Token};

fn setup() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

fn parse<'src>(path: &Path, source: &'src str) -> Result<Cst<'src>> {
    let mut diags = vec![];

    let (tokens, ranges) = tokenize(Token::lexer(source), &mut diags);
    let cst = Parser::parse(source, tokens, ranges, &mut diags);

    let config = Config::default();
    let writer = StandardStream::stderr(ColorChoice::Auto);
    let file   = SimpleFile::new(format!("{}", path.display()), &source);

    for diag in &diags {
        term::emit(&mut writer.lock(), &config, &file, diag).unwrap();
    }

    if diags.iter().any(|diag| diag.severity >= Severity::Error) {
        return Err(eyre!("{} was unable to be parsed", path.display()))
    }

    Ok(cst)
}

fn main() -> Result<(), Report> {
    setup()?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        std::process::exit(1);
    }

    let path   = Path::new(&args[1]);
    let source = std::fs::read_to_string(path)?;
    let cst    = parse(path, &source)?;

    println!("{cst}");

    Ok(())
}
