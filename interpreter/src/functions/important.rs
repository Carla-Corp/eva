use std::process::exit;

use crate::core::EvaValue;
use crate::statics::IMPORTANT_COUNTER;

pub fn service(args: Vec<EvaValue>) -> EvaValue {
    unsafe { IMPORTANT_COUNTER += 1; }
    let Some(value) = args.first() else {
        return EvaValue::Nil;
    };

    if *value == EvaValue::Nil {
        println!("\x1b[1;31mThe {}th \x1b[3;31mimportant\x1b[0m\x1b[1;31m value was nil\x1b[0m", unsafe { IMPORTANT_COUNTER });
        exit(1);
    }

    value.clone()
}
