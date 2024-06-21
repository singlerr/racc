use decl::racc;

fn main(){
    racc!{
        LETTER = [a-zA-Z]+: { return TOK_LETTER };
        IDENT = {LETTER}+: { return TOK_IDENT };
    }
}

#[cfg(test)]
mod tests {
    use decl::racc;

    #[test]
    fn it_works() {

    }
}
