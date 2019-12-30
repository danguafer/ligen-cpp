use crate::ast::{EMemberFunctionImpl, Args, MemberFunctionImpl, FunctionStatement, Expression, Identifier, FunctionBlock, FunctionCall, Modifier, Assignment, Type};
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

        let is_moving_self = if let Some(typ) = &method.inputs.self_type {
            args.args.insert(0, Identifier::new("*this"));
            match &typ.modifier {
                ligen_core::TypeModifier::None => true,
                _ => false
            }
        } else { false };

        let function_call = FunctionCall::new(Identifier::new(&format!("{}_{}", method.owner.identifier.name, method.identifier.name)), args);
        if let Some(typ) = &method.output.typ {
            let expression = if !typ.is_atomic() {
                let args = Args::new(vec![Identifier::new(&format!("{}", function_call))]);
                let constructor_call = FunctionCall::new(Identifier::new(&typ.identifier.name), args);
                Expression::FunctionCall(constructor_call)
            } else {
                Expression::FunctionCall(function_call)
            };
            statements.push(FunctionStatement::Assignment(Assignment::new(Identifier::new("auto ret_value"), expression)));
            if is_moving_self {
                statements.push(FunctionStatement::Assignment(Assignment::new(Identifier::new("this->self"), Expression::Identifier(Identifier::new("0")))));
            }
            for input in &method.inputs.inputs {
                match &input.typ.modifier {
                    ligen_core::TypeModifier::None => {
                        if (!input.typ.is_atomic()) {
                            statements.push(FunctionStatement::Assignment(Assignment::new(Identifier::new(&format!("{}.self", input.identifier.name)), Expression::Identifier(Identifier::new("0")))))
                        }
                    },
                    _ => ()
                }
            }
            statements.push(FunctionStatement::Return(Expression::Identifier(Identifier::new("ret_value"))));
        } else {
            statements.push(FunctionStatement::Expression(Expression::FunctionCall(function_call)));
        }
        let mut function_block = FunctionBlock::new(statements);
        MemberFunctionImpl::new(class_identifier, MemberFunctionGenerator::generate(&method, sized_integer), function_block)
    }
}
