use crate::ast::{Identifier, Parameter, Parameters, Modifier, Type};
use crate::generators::{TypeGenerator};

pub struct ParameterGenerator {}

impl ParameterGenerator {
    pub fn generate(input : &ligen_core::Input, sized_integer : bool) -> Parameter {
        let mut typ = TypeGenerator::generate(&input.typ, sized_integer);
        if !typ.is_atomic() {
            if let Modifier::None = typ.modifier {
                typ.modifier = Modifier::Move
            }
        }
        Parameter::new(typ, Identifier::new(&input.identifier.name))
    }
}

pub struct ParametersGenerator {}

impl ParametersGenerator {
    pub fn generate(inputs : &ligen_core::Inputs, sized_integer : bool) -> Parameters {
        let mut parameters = Vec::new();
        if let Some(self_type) = &inputs.self_type {
            let is_mutable = match &self_type.modifier {
                ligen_core::TypeModifier::Reference(reference) => reference.is_mutable,
                ligen_core::TypeModifier::Pointer(reference) => reference.is_mutable,
                ligen_core::TypeModifier::None => false
            };
            let typ = Type::new(!is_mutable, Identifier::new(&self_type.identifier.name), Modifier::None);
            let identifier = Identifier::new("self");
            parameters.push(Parameter::new(typ, identifier));
        }
        for input in &inputs.inputs {
            parameters.push(ParameterGenerator::generate(&input, sized_integer));
        }
        if let Some(_ty) = &inputs.self_type {
            parameters.remove(0);
        }
        Parameters::new(parameters)
    }
}
