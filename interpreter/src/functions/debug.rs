use crate::core::EvaValue;

pub fn service(args: Vec<EvaValue>) -> EvaValue {
    let Some(value) = args.first() else {
        return EvaValue::Nil;
    };

    println!("debug: {value:#?}");
    return EvaValue::Nil;
}
