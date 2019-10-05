use crate::ast::{DestructorImpl, Identifier};

pub struct DestructorImplGenerator {}

impl DestructorImplGenerator {
    pub fn generate(method : &ligen_core::Method) -> DestructorImpl {
        DestructorImpl::new(Identifier::new(&method.owner.identifier.name))
    }
}
