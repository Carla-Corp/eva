use crate::core::EvaValue;

pub const TOTAL_FUNCTIONS: usize = 3;
pub static FUNCTIONS: [(&'static str, fn(Vec<EvaValue>) -> EvaValue); TOTAL_FUNCTIONS] = [
    ("debug", crate::functions::debug::service),
    ("env", crate::functions::env::service),
    ("if", crate::functions::ternary::service),
];
