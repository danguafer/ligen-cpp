use crate::ast::{Identifier, Modifier};
use std::fmt;

pub struct Type {
    pub constness : bool,
    pub identifier : Identifier,
    pub modifier : Modifier
}

impl Type {
    pub fn new(constness : bool, identifier : Identifier, modifier : Modifier) -> Self {
        Self {
            constness,
            identifier,
            modifier
        }
    }

    pub fn is_atomic(&self) -> bool {
        match self.identifier.name.as_str() {
            "bool" | "uint8_t" | "uint16_t" | "uint32_t" | "uint64_t" | "int8_t" | "int16_t" | "int32_t" | "int64_t" | "char" | "unsigned char" | "short" | "unsigned short" | "int" | "unsigned int" | "long" | "unsigned long" | "float" | "double" => true,
            _ => false
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.constness {
            write!(f, "const ")?;
        }
        write!(f, "{}{}", self.identifier, self.modifier)
    }
}
