#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Punctuator {
    /// "!"
    Not,
    /// "!="
    NotEquals,
    /// "!=="
    NotEqualEquals,

    /// "~"
    BitwiseNot,

    /// "%"
    Modulo,
    /// "%="
    ModuleEquals,

    /// "&"
    BitwiseAnd,
    /// "&="
    BitwiseAndEquals,
    /// "&&"
    LogicalAnd,

    /// "|"
    BitwiseOr,
    /// "|="
    BitwiseOrEquals,
    /// "||"
    LogicalOr,

    /// "^"
    BitwiseXOr,
    /// "^="
    BitwiseXOrEquals,


    /// "("
    OpenParentheses,
    /// ")"
    CloseParentheses,

    /// "{"
    OpenCurly,
    /// "}"
    CloseCurly,

    /// "["
    OpenBracket,
    /// "]"
    CloseBracket,

    /// "+"
    Plus,
    /// "++"
    PlusPlus,
    /// "+=",
    PlusEquals,

    /// "-"
    Minus,
    /// "--"
    MinusMinus,
    /// "-="
    MinusEquals,

    /// "*"
    Multiply,
    /// "*="
    MultiplyEquals,

    /// "**"
    Power,
    /// "**="
    PowerEquals,

    /// "/"
    Divide,
    /// "/="
    DivideEquals,

    /// "<"
    LessThan,
    /// "<="
    LessThanOrEqual,

    /// "<<"
    ShiftLeft,
    /// "<<="
    ShiftLeftEquals,
    /// "<<<"
    ShiftLeftUnsigned,
    /// "<<<="
    ShiftLeftUnsignedEquals,

    /// ">"
    GreaterThan,
    /// ">="
    GreaterThanEquals,

    /// ">>"
    ShiftRight,
    /// ">>="
    ShiftRightEquals,
    /// ">>>"
    ShiftRightUnsigned,
    /// ">>>="
    ShiftRightUnsignedEquals,

    /// "="
    Assignment,
    /// "=="
    DoubleEquals,
    /// "==="
    TripleEquals,


    /// "."
    Period,
    /// "..."
    TriplePeriod,
    /// ","
    Comma,

    /// ";"
    Semicolon,
    /// ":"
    Colon,
    /// "::"
    DoubleColon,

    /// "?"
    QuestionMark,
    /// "??"
    DoubleQuestionMark,

    /// "=>"
    Arrow
}
impl Punctuator {
    pub fn as_str(self) -> &'static str {
        match self {
            Punctuator::Plus => "+",
            Punctuator::Minus => "-",
            Punctuator::Multiply => "*",
            Punctuator::Divide => "/",
            Punctuator::Period => ".",
            Punctuator::Comma => ",",
            Punctuator::Semicolon => ";",
        }
    }

}

