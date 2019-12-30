use crate::ast::{MemberFunction, Identifier, Parameters};
use crate::generators::{TypeGenerator, ParametersGenerator};

pub struct MemberFunctionGenerator {}

impl MemberFunctionGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> MemberFunction {
        let return_type = if let Some(typ) = &method.output.typ { TypeGenerator::generate(&typ, sized_integer) } else { TypeGenerator::void() };
        let identifier = Identifier::new(&method.identifier.name);
        let parameters = ParametersGenerator::generate(&method.inputs, sized_integer);
        let constness = if let Some(typ) = &method.inputs.self_type {
            match &typ.modifier {
                ligen_core::TypeModifier::Reference(reference) => !reference.is_mutable,
                ligen_core::TypeModifier::Pointer(reference) => !reference.is_mutable,
                ligen_core::TypeModifier::None => false
            }
        } else {
            false
        };
        MemberFunction::new(return_type, identifier, parameters, constness)
    }
}
