#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // int
    Inteiro(i64),
    // float
    Real(f64),
    // str
    Texto(String),
    // bool
    Logico(bool),
    // tuple
    Tupla,
    // list
    Lista,
    // set
    Conjunto,
    // dict
    Dicionario,
    // def
    Fun,
    // class
    Classe,
    // print
    Imprima,
    // input
    Entrada,
    // if
    Se,
    // else
    SeNao,
    // Elif?
    OuEntaoSe,
    // and
    E,
    // or
    Ou,
    // not
    Nao,
    // del
    Remova,
    // in
    Em,
    // assert
    Verifique,
    // break
    Interrompa,
    // return
    Retorne,
    // None,
    Vazio,
    // continue
    Continue,
    // for
    ParaCada,
    // while
    Enquanto,
    // global
    Global,
    // try
    Tente,
    // except
    Exceto,
    // pass
    Passe,
    // raise
    Provoque,

    // identifier == variables
    Identifier(String),

    // symbols
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    // %
    Percent,
    Comment,
    Bang,
    // :
    Colon,
    Comma,
    Dot,
    Greater,
    Less,
    ParentOpen,
    ParentClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,

    // composite symbols
    EqualEqual,
    GreaterThan,
    LessThan,
    BangEqual,

    // etc
    WhiteSpace,
    EOF,
}