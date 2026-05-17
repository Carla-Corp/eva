use crate::core::EvaValue;
use crate::parser::Parser;

pub const TOTAL_FUNCTIONS: usize = 18;
pub static FUNCTIONS: [(&'static str, fn(&mut Parser, Vec<EvaValue>) -> EvaValue); TOTAL_FUNCTIONS] = [
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
    ("clamp", crate::functions::clamp::service),
    ("important", crate::functions::important::service),
    ("ref", crate::functions::reference::service),
    ("format", crate::functions::format::service),
    ("keys", crate::functions::keys::service),
    ("values", crate::functions::values::service),
    ("entries", crate::functions::entries::service),
    ("merge", crate::functions::merge::service),
];

pub static mut IMPORTANT_COUNTER: usize = 0;
