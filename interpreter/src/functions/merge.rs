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
        ( EvaValue::Map(first), EvaValue::Map(second) ) => {
            let mut data = first.clone();

            data.retain(|(key, _)| {
                !second.iter().any(|(k, _)| k == key)
            });

            data.extend(second.iter().cloned());

            EvaValue::Map(data)
        },

        _ => EvaValue::Nil,
    };
    resolve
}
