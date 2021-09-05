#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /

    GE, // >=
    GT, // >
    EQ, // ==
    LE, // <=
    LT, // <

    SemiColon,  // ;
    LeftParen,  // (
    RightParen, // )

    Assignment, // =

    If,
    Else,

    Int,

    Identifier, //标识符

    IntLiteral,    //整型字面量
    StringLiteral, //字符串字面量
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Identifier
    }
}
