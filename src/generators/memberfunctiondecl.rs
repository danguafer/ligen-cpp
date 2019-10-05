use crate::ast::{EMemberFunctionDecl, Delete, MemberFunctionDecl};
use crate::generators::{MemberFunctionGenerator, ConstructorDeclGenerator, DestructorDeclGenerator};

pub struct EMemberFunctionDeclGenerator {}

impl EMemberFunctionDeclGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> EMemberFunctionDecl {
        match method.identifier.name.as_str() {
            "new" => EMemberFunctionDecl::Constructor(ConstructorDeclGenerator::generate(&method, sized_integer), Delete::False),
            "destroy" => EMemberFunctionDecl::Destructor(DestructorDeclGenerator::generate(&method), Delete::False),
            _ => EMemberFunctionDecl::MemberFunction(MemberFunctionDeclGenerator::generate(&method, sized_integer), Delete::False)
        }
    }
}

pub struct MemberFunctionDeclGenerator {}

impl MemberFunctionDeclGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> MemberFunctionDecl {
        let staticness = if let Some(_ty) = &method.inputs.self_type { false } else { true };
        MemberFunctionDecl::new(staticness, MemberFunctionGenerator::generate(&method, sized_integer))
    }
}
