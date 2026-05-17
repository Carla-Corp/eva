use crate::core::*;
use crate::parser::Parser;

pub fn service(parser: &mut Parser, args: Vec<EvaValue>) -> EvaValue {
    let Some(value) = args.first() else {
        return EvaValue::Nil;
    };

    let EvaValue::OtherField(identifier) = value else {
        return EvaValue::Nil;
    };

    for data in parser.cache.iter() {
        match data {
            EvaCached::Field(ns, field_name, value) => {
                if ns == &parser.namespace && field_name == identifier {
                    return value.clone();
                }
            }
            _ => {}
        }
    }

    return EvaValue::Nil;
}
