use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Float(f64),
    Int(i128),
    String(String),
    Bool(bool),
    NONE,
}

impl Value {
    pub fn value_type(&self) -> String {
        match self {
            Self::Float(_) => "Float".into(),
            Self::Int(_) => "Int".into(),
            Self::String(_) => "String".into(),
            Self::Bool(b) => b.to_string(),
            Self::NONE => "NONE".to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(n) => write!(f, "{}", n),
            Self::Int(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::NONE => write!(f, "NONE"),
        }
    }
}
