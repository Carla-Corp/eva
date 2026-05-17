use crate::core::EvaValue;

pub fn service(args: Vec<EvaValue>) -> EvaValue {
    let Some(value) = args.first() else {
        return EvaValue::Nil;
    };

    let EvaValue::String(name) = value else {
        return EvaValue::Nil;
    };

    let Ok(data) = std::env::var(name) else {
        return EvaValue::Nil;
    };

    EvaValue::String(data)
}
