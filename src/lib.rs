use ligen::ligen;
use ligen_core::{Object, Files, File, Attributes, LiteralConverter};

pub mod ast;
pub mod generators;

use generators::*;

pub struct Generator {
    c_generator : ligen_c::Generator,
    pub sized_integer : bool
}

#[ligen]
impl Generator {
    pub fn new(attributes: &Attributes) -> Generator {
        let c_generator = ligen_c::Generator::new(&attributes);
        let sized_integer = attributes.get_named("sized_integer").as_bool(true);

        Generator {
            c_generator,
            sized_integer
        }
    }

    pub fn generate(&self, object: &Object) -> Files {
        let mut files = self.c_generator.generate(&object);

        let hpp = format!("{}", HPPGenerator::generate(&object, self.sized_integer));
        let cpp = format!("{}", CPPGenerator::generate(&object, self.sized_integer));
        let hpp = File::new(format!("cpp/include/{}.hpp", object.typ.identifier.name), hpp);
        let cpp = File::new(format!("cpp/src/{}.cpp", object.typ.identifier.name), cpp);

        files.push(hpp);
        files.push(cpp);
        files
    }
}
