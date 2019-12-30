use crate::ast::{Type, Identifier, Modifier};

pub struct TypeGenerator {}

impl TypeGenerator {
    pub fn void() -> Type {
        Type::new(false, Identifier::new("void"), Modifier::None)
    }

    pub fn generate(typ : &ligen_core::Type, sized_integer : bool) -> Type {
        let (constness, modifier) = match &typ.modifier {
            ligen_core::TypeModifier::Reference(reference) => (!reference.is_mutable && !typ.is_atomic(), Modifier::Reference),
            ligen_core::TypeModifier::Pointer(reference) => (!reference.is_mutable && !typ.is_atomic(), Modifier::Pointer),
            ligen_core::TypeModifier::None => (false, Modifier::None)
        };

        if typ.is_atomic() {
            let name = ligen_c::generators::TypeGenerator::translate_atomic(&typ.identifier.name, sized_integer);
            let identifier = Identifier::new(name);
            Type::new(constness, identifier, modifier)
        } else {
            Type::new(constness, Identifier::new(&typ.identifier.name), modifier)
        }
    }
}
