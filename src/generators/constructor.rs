use crate::ast::{Identifier, Parameters, Constructor};
use crate::generators::{ParametersGenerator};

pub struct ConstructorGenerator {}

impl ConstructorGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> Constructor {
        let identifier = Identifier::new(&method.owner.identifier.name);
        let parameters = ParametersGenerator::generate(&method.inputs, sized_integer);
        Constructor::new(identifier, parameters)
    }
}
