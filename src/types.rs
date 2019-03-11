use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[derive(Clone, Debug)]
pub struct DescribeStatement {
    pub name: String,
    pub containers: Vec<Container>,
    pub before: Option<TokenStream>,
    pub after: Option<TokenStream>,
    pub subject: Option<TokenStream>,
}

impl DescribeStatement {
    pub fn new(
        name: String,
        containers: Vec<Container>,
        before: Option<TokenStream>,
        after: Option<TokenStream>,
        subject: Option<TokenStream>,
    ) -> Self {
        DescribeStatement {
            name,
            containers,
            before,
            after,
            subject,
        }
    }

    pub fn expands(describe_statements: Vec<Self>) -> TokenStream {
        let mut stream = TokenStream::new();

        for statement in describe_statements {
            let token_stream = statement.expand();
            stream = stream.into_iter().chain(token_stream.into_iter()).collect();
        }

        stream
    }

    // why result?
    pub fn expand(&self) -> TokenStream {
        let mod_name = Ident::new(&self.name, Span::call_site());
        let before = self.before.clone().unwrap_or(TokenStream::new());
        let after = self.before.clone().unwrap_or(TokenStream::new());

        let subject = if let Some(subject) = self.subject.clone() {
            quote! {
                // FIXME want to replace it when evaluated
                // instead of making it a variable
                let subject = (#subject);
            }
        } else {
            TokenStream::new()
        };

        let containers = Self::expand_containers(&self.containers, before, after, subject);

        quote! {
            mod #mod_name {
                #containers
            }
        }
    }

    fn expand_containers(
        containers: &Vec<Container>,
        before: TokenStream,
        after: TokenStream,
        subject: TokenStream,
    ) -> TokenStream {
        let mut stream = TokenStream::new();

        for container in containers {
            let token_stream = match container {
                Container::Describe(describe) => describe.expand(),
                Container::Test(test) => {
                    test.expand(before.clone(), after.clone(), subject.clone())
                }
            };

            stream = stream.into_iter().chain(token_stream.into_iter()).collect();
        }
        stream
    }
}

impl PartialEq for DescribeStatement {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

#[derive(Clone, Debug)]
pub struct Test {
    pub name: String,
    pub container: TokenStream,
}

impl Test {
    pub fn expand(
        &self,
        before: TokenStream,
        after: TokenStream,
        subject: TokenStream,
    ) -> TokenStream {
        let test_name = Ident::new(&self.name, Span::call_site());
        let container = self.container.clone();

        quote! {
            #[test]
            fn #test_name() {
                #before
                #subject

                #container

                #after
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Callbacks {
    pub before: Option<TokenStream>,
    pub after: Option<TokenStream>,
    pub subject: Option<TokenStream>,
}

#[derive(Clone, Debug)]
pub enum Container {
    Describe(DescribeStatement),
    Test(Test),
}
