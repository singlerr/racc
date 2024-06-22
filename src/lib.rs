use decl::lex;

fn main(){
    lex!{
        LETTER = [a-zA-Z]+: { return TOK_LETTER };
        IDENT = {LETTER}+: { return TOK_IDENT };
    }
}

#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {

    }
}
