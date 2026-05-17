use crate::core::EvaValue;
use crate::parser::Parser;

pub fn service(_: &mut Parser, args: Vec<EvaValue>) -> EvaValue {
    let Some(value) = args.first() else {
        return EvaValue::Nil;
    };

    let EvaValue::Map(data) = value else {
        return EvaValue::Nil;
    };

    if data.len() == 0 {
        return EvaValue::Nil;
    }

    let keys: Vec<EvaValue> = data.iter().map(|(k, _)| EvaValue::String(k.clone())).collect();
    EvaValue::List(keys)
}
