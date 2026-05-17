use crate::core::EvaValue;

pub fn service(args: Vec<EvaValue>) -> EvaValue {
    let Some(value) = args.get(0) else {
        return EvaValue::Nil;
    };

    let Some(truth) = args.get(1) else {
        return EvaValue::Nil;
    };

    let Some(lie) = args.get(2) else {
        return EvaValue::Nil;
    };

    let resolve = match value {
        EvaValue::Bool(true) => truth.clone(),
        EvaValue::Bool(false) => lie.clone(),

        EvaValue::Number(0f64) => lie.clone(),
        EvaValue::String(str) if str.is_empty() => lie.clone(),
        EvaValue::List(arr) if arr.is_empty() => lie.clone(),
        EvaValue::Map(map) if map.is_empty() => lie.clone(),
        EvaValue::Nil => lie.clone(),

        _ => truth.clone(),
    };

    resolve
}
