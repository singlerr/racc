use proc_macro::token_stream::IntoIter;
use proc_macro::{Delimiter, Group, Ident, Punct, TokenStream, TokenTree};

pub struct Def{
    pub name: String,
    pub def: TokenStream,
    pub action: TokenStream
}

pub struct DefParser{
    pointer: IntoIter
}

impl Def {
    fn new(name:String, def: TokenStream, action: TokenStream) -> Def {
        Def {
            name,
            def,
            action
        }
    }
}

impl DefParser{
    pub fn new(stream: &mut TokenStream) -> DefParser {
        let pointer = stream.clone().into_iter();
        DefParser{
            pointer
        }
    }

    pub fn next_def(&mut self) -> Option<Def>{
        let def_name = self.expect_ident()?;
        self.expect_punct('=')?;

        let mut expr = TokenStream::new();

        loop {
            let expr_token = self.next_token()?;
            if let TokenTree::Punct(p) = &expr_token && p.as_char() == ':'{
                break;
            }
            &expr.extend_one(expr_token);
        }

        let mut expr_action = TokenStream::new();
        loop {
            let expr_token = self.next_token()?;

            if let TokenTree::Punct(p) = &expr_token && p.as_char() == ';'{
                break;
            }

            &expr_action.extend_one(expr_token);
        }
        Some(Def{
            name: def_name.to_string(),
            def: expr,
            action: expr_action
        })
    }

    fn next_token(&mut self) -> Option<TokenTree> {
        self.pointer.next()
    }

    fn expect_ident(&mut self) -> Option<Ident>{
        let token = self.next_token()?;
        if let TokenTree::Ident(ident) = token{
            Some(ident)
        }else{
            None
        }
    }

    fn expect_group(&mut self) -> Option<Group>{
        let token = self.next_token()?;
        if let TokenTree::Group(group) = token{
            Some(group)
        }else{
            None
        }
    }

    fn expect_punct(&mut self, expected_value: char) -> Option<Punct>{
        let token = self.next_token()?;
        if let TokenTree::Punct(punct) = token && punct.as_char() == expected_value{
            Some(punct)
        }else{
            None
        }
    }
}