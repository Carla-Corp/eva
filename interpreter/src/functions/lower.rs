use crate::core::EvaValue;

pub fn service(args: Vec<EvaValue>) -> EvaValue {
    let Some(value) = args.first() else {
        return EvaValue::Nil;
    };

    let EvaValue::String(data) = value else {
        return EvaValue::Nil;
    };

    EvaValue::String(data.to_lowercase())
}
