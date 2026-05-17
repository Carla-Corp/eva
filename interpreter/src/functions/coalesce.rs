use crate::core::EvaValue;
use crate::parser::Parser;
use crate::statics::COLASECE_COUNTER;

pub fn service(_: &mut Parser, args: Vec<EvaValue>) -> EvaValue {
    unsafe { COLASECE_COUNTER += 1; }

    for arg in args {
        if let EvaValue::Nil = arg {
            continue;
        }
        return arg;
    }

    println!("\x1b[1;33mThe {}th \x1b[3;33mcoalesce\x1b[0m\x1b[1;33m value was nil. Add it a `important` call to abort your code when it happens.\x1b[0m", unsafe { COLASECE_COUNTER });
    EvaValue::Nil
}
