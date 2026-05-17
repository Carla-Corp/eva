use crate::core::EvaValue;

pub const TOTAL_FUNCTIONS: usize = 10;
pub static FUNCTIONS: [(&'static str, fn(Vec<EvaValue>) -> EvaValue); TOTAL_FUNCTIONS] = [
    ("debug", crate::functions::debug::service),
    ("env", crate::functions::env::service),
    ("if", crate::functions::ternary::service),
    ("else", crate::functions::default::service),
    ("lower", crate::functions::lower::service),
    ("upper", crate::functions::upper::service),
    ("trim", crate::functions::trim::service),
    ("contains", crate::functions::contains::service),
    ("startswith", crate::functions::starts_with::service),
    ("endswith", crate::functions::ends_with::service),
];
