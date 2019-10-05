use crate::ast::{AST, Statement, Macro, Identifier};
use crate::generators::{ClassImplGenerator};

pub struct CPPGenerator {}

impl CPPGenerator {
    pub fn generate(object : &ligen_core::Object, sized_integer : bool) -> AST {
        let mut statements = Vec::new();
        statements.push(Statement::Macro(Macro::new(Identifier::new("include"), &format!("<{}.hpp>", object.typ.identifier.name))));
        statements.push(Statement::Macro(Macro::new(Identifier::new("include"), "<utility>")));
        statements.append(&mut ClassImplGenerator::generate(&object, sized_integer).statements);
        AST::new(statements)
    }
}
