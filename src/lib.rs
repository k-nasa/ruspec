extern crate proc_macro;
extern crate syn;

use colored::*;
use proc_macro2::TokenStream;
use types::DescribeStatement;

mod parser;
mod types;

/// example syntax
/// ```rust
/// ruspec! {
///   describe "test name" {
///     before {
///        let context = 5;
///     }
///
///     it "should equal 5" {
///       assert_eq!(context, 5)
///     }
///
///     describe "inner describe" {
///       subject { context + 8 }
///
///       it "should equal 13" {
///         assert_eq!(subject, 13)
///       }
///     }
///   }
/// }
/// ```
///
/// this code is expande it
///```rust
/// mod test_name {
///     #[test]
///     fn should_equal_5() {
///         let context = 5;
///         assert_eq!(context, 5)
///     }
///
///     mod inner_describe {
///         #[test]
///         fn should_equal_13() {
///             let context = 5;
///             assert_eq!(context + 8, 13)
///         }
///     }
/// }
///```
///
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
