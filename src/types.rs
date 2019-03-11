use proc_macro2::TokenStream;

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
