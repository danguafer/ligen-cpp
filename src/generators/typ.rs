use crate::ast::{Type, Identifier, Modifier};

pub struct TypeGenerator {}

impl TypeGenerator {
    pub fn void() -> Type {
        Type::new(false, Identifier::new("void"), Modifier::None)
    }

    pub fn generate(ty : &ligen_core::Type, sized_integer : bool) -> Type {
        let (constness, modifier) = if let Some(reference) = &ty.reference {
            (!reference.is_mutable && !ty.is_atomic(), Modifier::Reference)
        } else {
            (false, Modifier::None)
        };

        if ty.is_atomic() {
            let name = ligen_c::generators::TypeGenerator::translate_atomic(&ty.identifier.name, sized_integer);
            let identifier = Identifier::new(name);
            Type::new(constness, identifier, Modifier::None)
        } else {
            Type::new(constness, Identifier::new(&ty.identifier.name), modifier)
        }
    }
}
