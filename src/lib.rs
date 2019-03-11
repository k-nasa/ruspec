extern crate proc_macro;
extern crate syn;

use colored::*;
use proc_macro2::TokenStream;
use types::DescribeStatement;

mod parser;
mod types;

#[proc_macro]
pub fn ruspec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let expanded = match _ruspec(input) {
        Ok(token_stream) => token_stream,
        Err(e) => {
            eprintln!("{}: {}", "error".red().bold(), e);
            std::process::exit(1);
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn _ruspec(input: proc_macro2::TokenStream) -> Result<TokenStream, failure::Error> {
    let describe_statements = parser::Parser::new(input).parse()?;
    Ok(DescribeStatement::expands(describe_statements))
}
