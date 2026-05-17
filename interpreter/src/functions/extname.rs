use crate::core::EvaValue;
use crate::parser::Parser;

pub fn service(_: &mut Parser, args: Vec<EvaValue>) -> EvaValue {
    let Some(value) = args.first() else {
        return EvaValue::Nil;
    };

    let EvaValue::String(name) = value else {
        return EvaValue::Nil;
    };

    let Ok(data) = std::path::absolute(name) else {
        return EvaValue::Nil;
    };

    let Some(ext) = data.extension() else {
        return EvaValue::Nil;
    };

    EvaValue::String(ext.to_str().unwrap().into())
}
