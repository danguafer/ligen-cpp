use crate::ast::{EMemberFunctionDecl, Access};

pub enum ClassStatement {
    MemberFunction(EMemberFunctionDecl),
    Access(Access),
    Uncategorized(String)
}

use std::fmt;
impl fmt::Display for ClassStatement {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClassStatement::MemberFunction(value) => write!(f, "{}", value),
            ClassStatement::Access(value) => write!(f, "{}:", value),
            ClassStatement::Uncategorized(value) => write!(f, "{}", value)
        }
    }
}
