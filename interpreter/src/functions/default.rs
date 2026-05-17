use crate::core::EvaValue;
use crate::parser::Parser;

pub fn service(_: &mut Parser, args: Vec<EvaValue>) -> EvaValue {
    let Some(value) = args.get(0) else {
        return EvaValue::Nil;
    };

    let Some(lie) = args.get(1) else {
        return EvaValue::Nil;
    };

    let resolve = match value {
        EvaValue::Bool(true) => value.clone(),
        EvaValue::Bool(false) => lie.clone(),

        EvaValue::Number(0f64) => lie.clone(),
        EvaValue::String(str) if str.is_empty() => lie.clone(),
        EvaValue::List(arr) if arr.is_empty() => lie.clone(),
        EvaValue::Map(map) if map.is_empty() => lie.clone(),
        EvaValue::Nil => lie.clone(),

        _ => value.clone(),
    };

    resolve
}
