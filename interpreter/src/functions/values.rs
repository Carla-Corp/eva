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

    let values: Vec<EvaValue> = data.iter().map(|(_, v)| v.clone()).collect();
    EvaValue::List(values)
}
