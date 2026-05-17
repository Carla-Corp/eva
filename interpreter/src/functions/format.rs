use crate::core::EvaValue;
use crate::parser::Parser;

pub fn service(_: &mut Parser, args: Vec<EvaValue>) -> EvaValue {
    let Some(first) = args.get(0) else {
        return EvaValue::Nil;
    };

    let EvaValue::String(mut format) = first.clone() else {
        return EvaValue::Nil;
    };

    let mut rest = args.into_iter().skip(1).collect::<Vec<_>>();
    while format.contains("{}") {
        let r = rest.clone();
        let Some(arg) = r.first() else {
            break;
        };

        rest.remove(0);

        let string = match arg {
            EvaValue::String(s) => s.clone(),
            EvaValue::Number(n) => n.to_string(),
            EvaValue::Nil => "nil".to_string(),
            EvaValue::Map(_) => "MAP()".to_string(),
            EvaValue::List(_) => "LIST()".to_string(),
            EvaValue::OtherField(_) => "ADDR()".to_string(),
            EvaValue::Bool(true) => "true".to_string(),
            EvaValue::Bool(false) => "false".to_string(),
        };

        format = format.replacen("{}", &string, 1);
    }

    EvaValue::String(format.clone())
}
