use crate::ast::{AST, Statement, Macro, Identifier};
use crate::generators::{ClassDeclGenerator};

pub struct HPPGenerator {}

impl HPPGenerator {
    pub fn generate(object : &ligen_core::Object, sized_integer : bool) -> AST {
        let name = &object.typ.identifier.name;
        let mut ast = Vec::new();
        ast.push(Statement::Macro(Macro::new(Identifier::new("pragma"), "once")));
        ast.push(Statement::Macro(Macro::new(Identifier::new("define"), &format!("{name} C{name}", name = name))));
        ast.push(Statement::Macro(Macro::new(Identifier::new("include"), &format!("<{}.h>", name))));
        ast.push(Statement::Macro(Macro::new(Identifier::new("undef"), &format!("{}", name))));
        ast.push(Statement::ClassDecl(ClassDeclGenerator::generate(&object, sized_integer)));

        AST::new(ast)
    }
}
