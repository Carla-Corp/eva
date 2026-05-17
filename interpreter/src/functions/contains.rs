use crate::core::EvaValue;

fn contains_str(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }

    let h = haystack.as_bytes();
    let n = needle.as_bytes();

    if n.len() > h.len() {
        return false;
    }

    for i in 0..=h.len() - n.len() {
        if &h[i..i + n.len()] == n {
            return true;
        }
    }

    false
}

pub fn service(args: Vec<EvaValue>) -> EvaValue {
    let Some(first) = args.get(0) else {
        return EvaValue::Nil;
    };

    let Some(second) = args.get(1) else {
        return EvaValue::Nil;
    };

    let resolve = match (first, second) {
        ( EvaValue::String(first), EvaValue::String(second) ) => EvaValue::Bool(contains_str(&first, &second)),
        ( EvaValue::List(values), value ) => EvaValue::Bool(
            values.iter().any(|v| v == value)
        ),
        _ => EvaValue::Nil,
    };

    resolve
}
