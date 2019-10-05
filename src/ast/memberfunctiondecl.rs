use crate::ast::{ MemberFunction, ConstructorDecl, DestructorDecl };
use std::fmt;

pub enum Delete {
    True,
    False
}

impl fmt::Display for Delete {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Delete::True => write!(f, "delete"),
            Delete::False => write!(f, "")
        }
    }
}

pub enum EMemberFunctionDecl {
    Constructor(ConstructorDecl, Delete),
    Destructor(DestructorDecl, Delete),
    MemberFunction(MemberFunctionDecl, Delete)
}

impl fmt::Display for EMemberFunctionDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let delete = match self {
            EMemberFunctionDecl::Constructor(value, delete) => {
                write!(f, "{}", value)?;
                delete
            },
            EMemberFunctionDecl::Destructor(value, delete) => {
                write!(f, "{}", value)?;
                delete
            },
            EMemberFunctionDecl::MemberFunction(value, delete) => {
                write!(f, "{}", value)?;
                delete
            }
        };
        match delete {
            Delete::True => write!(f, " = {}", delete)?,
            _ => ()
        }
        write!(f, ";")
    }
}

pub struct MemberFunctionDecl {
    staticness : bool,
    member_function : MemberFunction
}

impl MemberFunctionDecl {
    pub fn new(staticness : bool, member_function : MemberFunction) -> Self {
        Self {
            staticness,
            member_function
        }
    }
}

impl fmt::Display for MemberFunctionDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.staticness {
            write!(f, "static ")?
        }
        write!(f, "{} {}({})", self.member_function.return_type, self.member_function.identifier, self.member_function.parameters)?;
        if self.member_function.constness {
            write!(f, " const")
        } else {
            write!(f, "")
        }
    }
}
