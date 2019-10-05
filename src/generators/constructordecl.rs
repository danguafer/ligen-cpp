use crate::ast::{ConstructorDecl};
use crate::generators::{ConstructorGenerator};

pub struct ConstructorDeclGenerator {}

impl ConstructorDeclGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> ConstructorDecl {
        ConstructorDecl::new(ConstructorGenerator::generate(&method, sized_integer))
    }
}
