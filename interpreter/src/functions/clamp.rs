use crate::core::EvaValue;

pub fn service(args: Vec<EvaValue>) -> EvaValue {
    let Some(first) = args.get(0) else {
        return EvaValue::Nil;
    };

    let Some(second) = args.get(1) else {
        return EvaValue::Nil;
    };

    let Some(third) = args.get(2) else {
        return EvaValue::Nil;
    };

    let resolve = match (first, second, third) {
        (EvaValue::Number(x), EvaValue::Number(y), EvaValue::Number(z)) => EvaValue::Number(x.clamp(*y, *z)),
        _ => EvaValue::Nil,
    };

    resolve
}
