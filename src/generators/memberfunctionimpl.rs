use crate::ast::{EMemberFunctionImpl, Args, MemberFunctionImpl, FunctionStatement, Expression, Identifier, FunctionBlock, FunctionCall};
use crate::generators::{MemberFunctionGenerator, ConstructorImplGenerator, DestructorImplGenerator};
use ligen_c::generators::{ArgsGenerator};

pub struct EMemberFunctionImplGenerator {}

impl EMemberFunctionImplGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> EMemberFunctionImpl {
        match method.identifier.name.as_str() {
            "new" => EMemberFunctionImpl::Constructor(ConstructorImplGenerator::generate(&method, sized_integer)),
            "destroy" => EMemberFunctionImpl::Destructor(DestructorImplGenerator::generate(&method)),
            _ => EMemberFunctionImpl::MemberFunction(MemberFunctionImplGenerator::generate(&method, sized_integer))
        }
    }
}

pub struct MemberFunctionImplGenerator {}

impl MemberFunctionImplGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> MemberFunctionImpl {
        let class_identifier = Identifier::new(&method.owner.identifier.name);
        let mut statements = Vec::new();
        let mut args = ArgsGenerator::generate(&method.inputs);
        if let Some(_typ) = &method.inputs.self_type {
            args.args.insert(0, Identifier::new("*this"));
        }
        let function_call = FunctionCall::new(Identifier::new(&format!("{}_{}", method.owner.identifier.name, method.identifier.name)), args);
        if let Some(typ) = &method.output.typ {
            if typ.identifier.name == method.owner.identifier.name {
                let args = Args::new(vec![Identifier::new(&format!("{}", function_call))]);
                let constructor_call = FunctionCall::new(Identifier::new(&method.owner.identifier.name), args);
                statements.push(FunctionStatement::Return(Expression::FunctionCall(constructor_call)));
            } else {
                statements.push(FunctionStatement::Return(Expression::FunctionCall(function_call)));
            }
        } else {
            statements.push(FunctionStatement::Expression(Expression::FunctionCall(function_call)));
        }
        let mut function_block = FunctionBlock::new(statements);
        MemberFunctionImpl::new(class_identifier, MemberFunctionGenerator::generate(&method, sized_integer), function_block)
    }
}
