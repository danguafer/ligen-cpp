use crate::ast::{ MemberFunction, Identifier, ConstructorImpl, DestructorImpl, Modifier,FunctionBlock };
use std::fmt;

pub enum EMemberFunctionImpl {
    Constructor(ConstructorImpl),
    Destructor(DestructorImpl),
    MemberFunction(MemberFunctionImpl)
}

impl fmt::Display for EMemberFunctionImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EMemberFunctionImpl::Constructor(value) => write!(f, "{}", value),
            EMemberFunctionImpl::Destructor(value) => write!(f, "{}", value),
            EMemberFunctionImpl::MemberFunction(value) => write!(f, "{}", value)
        }
    }
}

pub struct MemberFunctionImpl {
    class_identifier : Identifier,
    member_function : MemberFunction,
    function_block : FunctionBlock
}

impl MemberFunctionImpl {
    pub fn new(class_identifier : Identifier, member_function : MemberFunction, function_block : FunctionBlock) -> Self {
        Self {
            class_identifier,
            member_function,
            function_block
        }
    }
}

impl fmt::Display for MemberFunctionImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}::{}({})", self.member_function.return_type, self.class_identifier, self.member_function.identifier, self.member_function.parameters)?;
        if self.member_function.constness {
            write!(f, " const")?
        }
        write!(f, " {}", self.function_block)
    }
}
