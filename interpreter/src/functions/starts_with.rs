use crate::core::EvaValue;
use crate::parser::Parser;

pub fn service(_: &mut Parser, args: Vec<EvaValue>) -> EvaValue {
    let Some(first) = args.get(0) else {
        return EvaValue::Nil;
    };

    let Some(second) = args.get(1) else {
        return EvaValue::Nil;
    };

    let resolve = match (first, second) {
        ( EvaValue::String(first), EvaValue::String(second) ) => EvaValue::Bool(first.starts_with(second)),
        ( EvaValue::List(values), starts ) => EvaValue::Bool(
            values.first().unwrap_or_else(|| &EvaValue::Nil) == starts
        ),
        _ => EvaValue::Nil,
    };

    resolve
}
