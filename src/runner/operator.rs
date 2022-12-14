use crate::runner::operator::add::Add;
use crate::runner::operator::and::And;
use crate::runner::operator::call::Call;
use crate::runner::operator::condition::Condition;
use crate::runner::operator::deref::Deref;
use crate::runner::operator::divide::Divide;
use crate::runner::operator::equals::Equals;
use crate::runner::operator::function::Function;
use crate::runner::operator::greater::Greater;
use crate::runner::operator::greater_equals::GreaterEquals;
use crate::runner::operator::include::Include;
use crate::runner::operator::less::Less;
use crate::runner::operator::less_equals::LessEquals;
use crate::runner::operator::multiply::Multiply;
use crate::runner::operator::not::Not;
use crate::runner::operator::not_equals::NotEquals;
use crate::runner::operator::or::Or;
use crate::runner::operator::pointer::Pointer;
use crate::runner::operator::set::Set;
use crate::runner::operator::subtract::Subtract;
use crate::runner::operator::syscall::SysCall;
use crate::runner::value::Value;
use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;

mod add;
mod and;
mod call;
mod condition;
mod deref;
mod divide;
mod equals;
mod function;
mod greater;
mod greater_equals;
mod include;
mod less;
mod less_equals;
mod multiply;
mod not;
mod not_equals;
mod or;
mod pointer;
mod set;
mod subtract;
mod syscall;

pub trait Operator {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError>;
}

pub fn get_operator(name: &str) -> Result<&dyn Operator, RuntimeError> {
    match name {
        Add::NAME => Ok(&Add {}),
        And::NAME => Ok(&And {}),
        Call::NAME => Ok(&Call {}),
        Condition::NAME => Ok(&Condition {}),
        Deref::NAME => Ok(&Deref {}),
        Divide::NAME => Ok(&Divide {}),
        Equals::NAME => Ok(&Equals {}),
        Function::NAME => Ok(&Function {}),
        Greater::NAME => Ok(&Greater {}),
        GreaterEquals::NAME => Ok(&GreaterEquals {}),
        Include::NAME => Ok(&Include {}),
        Less::NAME => Ok(&Less {}),
        LessEquals::NAME => Ok(&LessEquals {}),
        Multiply::NAME => Ok(&Multiply {}),
        NotEquals::NAME => Ok(&NotEquals {}),
        Or::NAME => Ok(&Or {}),
        Pointer::NAME => Ok(&Pointer {}),
        Set::NAME => Ok(&Set {}),
        Subtract::NAME => Ok(&Subtract {}),
        SysCall::NAME => Ok(&SysCall {}),
        Not::NAME => Ok(&Not {}),
        &_ => Err(RuntimeError::OperatorExpected(name.to_string())),
    }
}

pub fn cascade_eval<T>(expression: &Expression, context: &mut Context, f: fn(T, T) -> T) -> Value
where
    T: Into<Value>,
    T: From<Value>,
{
    let mut result = Runner::execute(&expression.get_branch(0), context)
        .ok_or(RuntimeError::NoValue)
        .unwrap()
        .into();

    for i in 1..expression.get_branches().len() {
        let tmp = Runner::execute(&expression.get_branch(i), context)
            .ok_or(RuntimeError::NoValue)
            .unwrap()
            .into();
        result = f(result, tmp);
    }

    result.into()
}

pub fn cascade_cmp(
    expression: &Expression,
    context: &mut Context,
    f: fn(Value, Value) -> bool,
) -> Value {
    let mut a = Runner::execute(&expression.get_branch(0), context)
        .ok_or(RuntimeError::NoValue)
        .unwrap();

    for i in 1..expression.get_branches().len() {
        let tmp = Runner::execute(&expression.get_branch(i), context)
            .ok_or(RuntimeError::NoValue)
            .unwrap();

        if !f(a, tmp.clone()) {
            return Value::Bool(false);
        }

        a = tmp;
    }

    Value::Bool(true)
}
