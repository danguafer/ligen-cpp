use crate::ast::{ClassDecl, Identifier, EMemberFunctionDecl, Inheritance, Access, ConstructorDecl, Constructor, Parameters, Parameter, Type, Delete, Modifier, MemberFunction, MemberFunctionDecl, ClassStatement};
use crate::generators::{EMemberFunctionDeclGenerator};

pub struct ClassDeclGenerator {}

impl ClassDeclGenerator {
    pub fn generate(object : &ligen_core::Object, sized_integer : bool) -> ClassDecl {
        let name = &object.typ.identifier.name;
        let identifier = Identifier::new(&name);
        let lower_name = name.to_lowercase();
        let c_name = format!("C{}", name);
        let inheritances = vec![Inheritance::new(Access::Public, Identifier::new(&c_name))];
        let mut statements = Vec::new();

        statements.push(ClassStatement::Access(Access::Public));

        {
            let identifier = Identifier::new(&name);
            let parameters = Parameters::new(vec![Parameter::new(Type::new(false, Identifier::new(&c_name), Modifier::None), Identifier::new(&lower_name))]);
            let constructor = Constructor::new(identifier, parameters);
            statements.push(ClassStatement::MemberFunction(EMemberFunctionDecl::Constructor(ConstructorDecl::new(constructor), Delete::False)));
        }

        {
            let identifier = Identifier::new(&name);
            let parameters = Parameters::new(vec![Parameter::new(Type::new(false, Identifier::new(&name), Modifier::Move), Identifier::new(&lower_name))]);
            let constructor = Constructor::new(identifier, parameters);
            statements.push(ClassStatement::MemberFunction(EMemberFunctionDecl::Constructor(ConstructorDecl::new(constructor), Delete::False)));
        }

        {
            let identifier = Identifier::new(&name);
            let parameters = Parameters::new(vec![Parameter::new(Type::new(true, Identifier::new(&name), Modifier::Reference), Identifier::new(&lower_name))]);
            let constructor = Constructor::new(identifier, parameters);
            statements.push(ClassStatement::MemberFunction(EMemberFunctionDecl::Constructor(ConstructorDecl::new(constructor), Delete::True)));
        }

        {
            let identifier = Identifier::new("operator=");
            let parameters = Parameters::new(vec![Parameter::new(Type::new(true, Identifier::new(&name), Modifier::Reference), Identifier::new(&lower_name))]);
            let return_type = Type::new(false, Identifier::new(&name), Modifier::Reference);
            let member_function = MemberFunction::new(return_type, identifier, parameters, false);
            statements.push(ClassStatement::MemberFunction(EMemberFunctionDecl::MemberFunction(MemberFunctionDecl::new(false, member_function), Delete::True)));
        }

        {
            let identifier = Identifier::new("operator=");
            let parameters = Parameters::new(vec![Parameter::new(Type::new(false, Identifier::new(&name), Modifier::Move), Identifier::new(&lower_name))]);
            let return_type = Type::new(false, Identifier::new(&name), Modifier::Reference);
            let member_function = MemberFunction::new(return_type, identifier, parameters, false);
            statements.push(ClassStatement::MemberFunction(EMemberFunctionDecl::MemberFunction(MemberFunctionDecl::new(false, member_function), Delete::False)));
        }

        for method in &object.methods {
            statements.push(ClassStatement::MemberFunction(EMemberFunctionDeclGenerator::generate(&method, sized_integer)));
        }
        ClassDecl::new(identifier, inheritances, statements)
    }
}
