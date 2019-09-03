use ligen::ligen;
use ligen_core::{Object, Method, Files, File};

pub struct Generator {
    c_generator : ligen_c::Generator
}

#[ligen]
impl Generator {
    pub fn new() -> Generator {
        let c_generator = ligen_c::Generator::new();
        Generator {
            c_generator
        }
    }
    pub fn generate(&self, object: &Object) -> Files {
        let mut files = self.c_generator.generate(&object);

        let (hpp, cpp) = self.generate_object(&object);
        let hpp = File::new(format!("cpp/include/{}.hpp", object.ty.identifier.name), hpp);
        let cpp = File::new(format!("cpp/src/{}.cpp", object.ty.identifier.name), cpp);
        files.push(hpp);
        files.push(cpp);
        files
    }

    fn generate_object(&self, object: &Object) -> (String, String) {
        let mut methods = (String::new(), String::new());
        for method in &object.methods {
            let method = self.generate_method(&method);
            methods.0.push_str(&format!("\n{}", method.0));
            methods.1.push_str(&format!("\n{}", method.1));
        };

        let hpp = format!(include_str!("object.hpp"), object = object.ty.identifier.name, methods = methods.0);
        let cpp = format!(include_str!("object.cpp"), object = object.ty.identifier.name, methods = methods.1);
        (hpp, cpp)
    }

    fn generate_method(&self, method: &Method) -> (String, String) {
        let return_type = self.c_generator.generate_output(&method.output);
        let object = &method.owner.identifier.name;
        let parameters = self.c_generator.generate_inputs(&method.inputs);
        let args = self.c_generator.generate_args(&method.inputs);

        let args = if method.inputs.is_associated {
            args
        } else {
            if args.len() > 0 {
                format!("self, {}", args)
            } else {
                format!("self")
            }
        };


        let ret = match &method.output.output {
                Some(_) => "return ",
                _ => ""
        };
        let method = &method.identifier.name;

        if method == "new" {
            let hpp = format!(include_str!("constructor.hpp"), parameters = parameters, object = object);
            let cpp = format!(include_str!("constructor.cpp"), method = method, parameters = parameters, object = object, args = args);
            (hpp, cpp)
        } else {
            let hpp = format!(include_str!("method.hpp"), return_type = return_type, method = method, parameters = parameters);
            let cpp = format!(include_str!("method.cpp"), return_type = return_type, method = method, parameters = parameters, object = object, ret = ret, args = args);
            (hpp, cpp)
        }
    }
}
