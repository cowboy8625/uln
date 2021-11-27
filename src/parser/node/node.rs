use std::fmt;
#[derive(Debug, PartialEq, Clone)]
pub enum Builtin {
    Print(Box<Expr>),
    PrintLn(Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Constant {
    String(String),
    Int(i128),
    Float(f64),
    Boolean(bool),
    Keyword(String),
    Builtin(Builtin),
}

impl Constant {
    pub fn name(&self) -> &str {
        match self {
            Self::String(_) => "String",
            Self::Int(_) => "Int",
            Self::Float(_) => "Float",
            Self::Boolean(_) => "Float",
            Self::Keyword(_) => "Keyword",
            Self::Builtin(b) => match b {
                Builtin::Print(_) => "Builtin Function Print",
                Builtin::PrintLn(_) => "Builtin Function PrintLn",
            },
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(float) => write!(f, "{}", float),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Keyword(s) => write!(f, "{}", s),
            Self::Builtin(_) => write!(f, "Print"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Minus,
    Bang,
    Plus,
    Multiply,
    Divide,
    GreaterThenEqual,
    LessThenEqual,
    GreaterThen,
    LessThen,
    Equality,
    NotEqual,
}

impl Operator {
    pub fn symbol(&self) -> &str {
        match self {
            Self::Minus => "-",
            Self::Bang => "!",
            Self::Plus => "+",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::GreaterThenEqual => ">=",
            Self::LessThenEqual => "<=",
            Self::GreaterThen => ">",
            Self::LessThen => "<",
            Self::Equality => "==",
            Self::NotEqual => "!=",
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Self::Minus => "Minus",
            Self::Bang => "Bang",
            Self::Plus => "Plus",
            Self::Multiply => "Multiply",
            Self::Divide => "Divide",
            Self::GreaterThenEqual => "GreaterThenEqual",
            Self::LessThenEqual => "LessThenEqual",
            Self::GreaterThen => "GreaterThen",
            Self::LessThen => "LessThen",
            Self::Equality => "Equality",
            Self::NotEqual => "NotEqual",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Constant(Constant),
    Unary {
        op: Operator,
        child: Box<Self>,
    },
    Binary {
        op: Operator,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    // condision  statement
    If(Box<Self>, Box<Self>),
    // condision  statement else statement
    IfElse(Box<Self>, Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    // name parameter statement
    // TODO: FIXME: First argument in Function Expression needs
    // to be a Constant.
    Function(String, Vec<String>, Box<Self>),
    Call(Box<Self>, Vec<Box<Self>>),
}

impl From<Constant> for Expr {
    fn from(c: Constant) -> Self {
        Self::Constant(c)
    }
}

impl From<Builtin> for Expr {
    fn from(b: Builtin) -> Self {
        Self::Constant(Constant::Builtin(b))
    }
}
