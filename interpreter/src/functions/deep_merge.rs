use crate::core::EvaValue;
use crate::parser::Parser;

fn deep_merge_value(left: EvaValue, right: EvaValue) -> EvaValue {
    match (left, right) {
        ( EvaValue::Map(mut left_map), EvaValue::Map(right_map) ) => {

            for (right_key, right_value) in right_map {
                match left_map.iter_mut().find(|(left_key, _)| *left_key == right_key) {
                    Some((_, left_value)) => {
                        let merged = deep_merge_value(left_value.clone(), right_value);
                        *left_value = merged;
                    }

                    None => left_map.push((right_key, right_value))
                }
            }

            EvaValue::Map(left_map)
        }

        (_, right) => right
    }
}

pub fn service(_: &mut Parser, args: Vec<EvaValue>) -> EvaValue {
    let Some(first) = args.get(0) else {
        return EvaValue::Nil;
    };

    let Some(second) = args.get(1) else {
        return EvaValue::Nil;
    };

    deep_merge_value(first.clone(), second.clone())
}
