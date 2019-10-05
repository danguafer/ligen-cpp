use crate::ast::{Macro, Function, ClassDecl, EMemberFunctionImpl};
use std::fmt;

pub enum Statement {
    Macro(Macro),
    Function(Function),
    ClassDecl(ClassDecl),
    MemberFunctionImpl(EMemberFunctionImpl),
    Uncategorized(String)
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Macro(value) => write!(f, "{}", value),
            Statement::Function(value) => write!(f, "{}", value),
            Statement::ClassDecl(value) => write!(f, "{}", value),
            Statement::MemberFunctionImpl(value) => write!(f, "{}", value),
            Statement::Uncategorized(value) => write!(f, "{}", value)
        }
    }
}
