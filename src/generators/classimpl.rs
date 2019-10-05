use crate::ast::{AST, Statement, FunctionStatement, MemberFunction, MemberFunctionImpl, Expression, Assignment, Type, Modifier, Parameter, Parameters, Constructor, FunctionBlock, ConstructorImpl, Identifier, EMemberFunctionImpl};
use crate::generators::{EMemberFunctionImplGenerator};

pub struct ClassImplGenerator {}

impl ClassImplGenerator {
    pub fn generate(object : &ligen_core::Object, sized_integer : bool) -> AST {
        let mut statements = Vec::new();

        let name = &object.typ.identifier.name;
        let lower_name = name.to_lowercase();
        let c_name = format!("C{}", name);

        { // CObject constructor
            let parameter = Parameter::new(Type::new(false, Identifier::new(&c_name), Modifier::None), Identifier::new(&lower_name));
            let parameters = Parameters::new(vec![parameter]);
            let constructor = Constructor::new(Identifier::new(&name), parameters);
            let initializers = Vec::new();
            let function_statements = vec![FunctionStatement::Assignment(Assignment::new(Identifier::new("this->self"), Expression::Identifier(Identifier::new(&format!("{}.self", lower_name)))))];
            let function_block = FunctionBlock::new(function_statements);
            let constructor = ConstructorImpl::new(constructor, initializers, function_block);
            statements.push(Statement::MemberFunctionImpl(EMemberFunctionImpl::Constructor(constructor)));
        }

        { // Move constructor
            let parameter = Parameter::new(Type::new(false, Identifier::new(&name), Modifier::Move), Identifier::new(&lower_name));
            let parameters = Parameters::new(vec![parameter]);
            let constructor = Constructor::new(Identifier::new(&name), parameters);
            let initializers = Vec::new();
            let function_statements = vec![FunctionStatement::Assignment(Assignment::new(Identifier::new("*this"), Expression::Identifier(Identifier::new(&format!("std::move({})", lower_name)))))];
            let function_block = FunctionBlock::new(function_statements);
            let constructor = ConstructorImpl::new(constructor, initializers, function_block);
            statements.push(Statement::MemberFunctionImpl(EMemberFunctionImpl::Constructor(constructor)));
        }

        { // Move constructor
            let parameter = Parameter::new(Type::new(false, Identifier::new(&name), Modifier::Move), Identifier::new(&lower_name));
            let parameters = Parameters::new(vec![parameter]);

            let return_type = Type::new(false, Identifier::new(&name), Modifier::Reference);
            let member_function = MemberFunction::new(return_type, Identifier::new("operator="), parameters, false);
            let mut function_statements = vec![FunctionStatement::Assignment(Assignment::new(Identifier::new("this->self"), Expression::Identifier(Identifier::new(&format!("{}.self", lower_name)))))];
            function_statements.push(FunctionStatement::Assignment(Assignment::new(Identifier::new(&format!("{}.self", lower_name)), Expression::Identifier(Identifier::new("0")))));
            function_statements.push(FunctionStatement::Return(Expression::Identifier(Identifier::new("*this"))));
            let function_block = FunctionBlock::new(function_statements);
            let member_function = MemberFunctionImpl::new(Identifier::new(&name), member_function, function_block);
            statements.push(Statement::MemberFunctionImpl(EMemberFunctionImpl::MemberFunction(member_function)));
        }

        for method in &object.methods {
            statements.push(Statement::MemberFunctionImpl(EMemberFunctionImplGenerator::generate(&method, sized_integer)));
        }
        AST::new(statements)
    }
}
