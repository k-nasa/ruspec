use self::Keyword::*;
use crate::types::{Callbacks, Container, DescribeStatement, Test};
use inflector::cases::snakecase::to_snake_case;
use proc_macro2::{TokenStream, TokenTree};

type DescribeStatements = Vec<DescribeStatement>;
type Containers = Vec<Container>;

#[derive(Debug, Clone)]
pub struct Parser {
    input: TokenStream,
    position: u32,
    current_token: Option<TokenTree>,
    peek_token: Option<TokenTree>,
}

impl Parser {
    pub fn new(input: TokenStream) -> Self {
        let mut iter = input.clone().into_iter();
        let current_token = iter.next();
        let peek_token = iter.next();

        Parser {
            input,
            position: 0,
            current_token,
            peek_token,
        }
    }

    pub fn parse(&mut self) -> Result<DescribeStatements, failure::Error> {
        let mut statements: DescribeStatements = Vec::new();

        while self
            .input
            .clone()
            .into_iter()
            .nth(self.position as usize)
            .is_some()
        {
            let keyword = if let Some(TokenTree::Ident(ident)) = self.clone().current_token {
                Keyword::from(ident.to_string())
            } else {
                failure::bail!("not found keyword in parse")
            };

            let statement = match keyword {
                DESCRIBE => self.parse_describe_statement()?,
                _ => {
                    failure::bail!("not found expected describe keyword");
                }
            };

            statements.push(statement);
        }

        Ok(statements)
    }

    fn parse_containers(&mut self) -> Result<Containers, failure::Error> {
        let mut containers: Containers = Vec::new();
        while self
            .input
            .clone()
            .into_iter()
            .nth(self.position as usize)
            .is_some()
        {
            let keyword = if let Some(TokenTree::Ident(ident)) = self.clone().current_token {
                Keyword::from(ident.to_string())
            } else {
                failure::bail!("not found keyword in parse_containers")
            };

            let container = match keyword {
                DESCRIBE => Container::Describe(self.parse_describe_statement()?),
                IT => Container::Test(self.parse_it_statement()?),
                _ => failure::bail!("not found expected describe keyword"),
            };

            containers.push(container);
            self.next_token();
        }

        Ok(containers)
    }

    fn parse_it_statement(&mut self) -> Result<Test, failure::Error> {
        self.next_token();

        let name = if let Some(TokenTree::Literal(literal)) = self.clone().current_token {
            to_snake_case(&literal.to_string())
        } else {
            failure::bail!("not found expected it string")
        };

        let container = if let Some(TokenTree::Group(group)) = self.clone().peek_token {
            group.stream()
        } else {
            failure::bail!("not found expected it block")
        };
        self.next_token();

        Ok(Test { name, container })
    }

    fn parse_describe_statement(&mut self) -> Result<DescribeStatement, failure::Error> {
        self.next_token();
        let name = if let Some(TokenTree::Literal(literal)) = self.clone().current_token {
            to_snake_case(&literal.to_string())
        } else {
            failure::bail!("not found expected describe string")
        };

        let stream = if let Some(TokenTree::Group(group)) = self.clone().peek_token {
            group.stream()
        } else {
            failure::bail!("not found expected describe block")
        };

        let mut parser = Self::new(stream);
        let mut callbacks = Callbacks::default();
        parser.parse_callback_statement(&mut callbacks)?;
        let containers = parser.parse_containers()?;
        self.next_token();
        self.next_token();

        Ok(DescribeStatement::new(
            name,
            containers,
            callbacks.before,
            callbacks.after,
            callbacks.subject,
        ))
    }

    fn parse_callback_statement(
        &mut self,
        callbacks: &mut Callbacks,
    ) -> Result<(), failure::Error> {
        let keyword = if let Some(TokenTree::Ident(ident)) = self.clone().current_token {
            Keyword::from(ident.to_string())
        } else {
            failure::bail!("not found expected keyword (it subject before after)")
        };
        if keyword == AFTER || keyword == BEFORE || keyword == SUBJECT {
            self.next_token();

            let stream = if let Some(TokenTree::Group(group)) = self.clone().current_token {
                group.stream()
            } else {
                failure::bail!("not found expected keyword")
            };

            match keyword {
                AFTER => callbacks.after = Some(stream),
                BEFORE => callbacks.before = Some(stream),
                SUBJECT => callbacks.subject = Some(stream),
                _ => (),
            }

            self.next_token();
            self.parse_callback_statement(callbacks)?;
        }

        Ok(())
    }

    fn next_token(&mut self) {
        self.position += 1;
        let peek_position = self.position as usize + 1;

        self.current_token = self.peek_token.clone();
        self.peek_token = self.input.clone().into_iter().nth(peek_position);
    }
}

#[derive(PartialEq, Debug)]
enum Keyword {
    DESCRIBE,
    BEFORE,
    AFTER,
    IT,
    SUBJECT,
    UNMATCH,
}

impl From<String> for Keyword {
    fn from(input: String) -> Self {
        match input.as_str() {
            "describe" | "context" => Keyword::DESCRIBE,
            "before" => Keyword::BEFORE,
            "after" => Keyword::AFTER,
            "it" => Keyword::IT,
            "subject" => Keyword::SUBJECT,
            _ => Keyword::UNMATCH,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate proc_macro;
    extern crate syn;

    use crate::parser::Parser;
    use crate::ruspec;
    use crate::types::{Container, DescribeStatement, Test};
    use proc_macro::TokenStream;
    use quote::quote;
    use std::str::FromStr;

    #[test]
    fn should_perse_describe() {
        let input = str_to_ts(
            r###"describe "test name" {
            it "hoge"{}
        }"###,
        );
        let mut parser = Parser::new(input);

        let expected = DescribeStatement {
            name: String::from("test_name"),
            before: None,
            after: None,
            subject: None,
            containers: vec![Container::Test(Test {
                name: "hoge".into(),
                container: proc_macro2::TokenStream::new(),
            })],
        };

        assert_eq!(parser.parse().unwrap(), vec![expected]);
    }

    // ts is token stream
    fn str_to_ts(input: &str) -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::from_str(input).unwrap()
    }
}
