pub enum CodeStatus {
    JustFailed = -1,
    Ok = 0,
    FailToOpenEvaFile = 1
}

pub type Error = String;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum EvaValue {
    String(String),
    OtherField(String),
    Number(f64),
    Bool(bool),
    List(Vec<EvaValue>),
    Map(Vec<(String, EvaValue)>),

    #[default]
    Nil
}

#[derive(Clone, Default)]
pub enum EvaCached {
    Field(String, String, EvaValue),

    #[default]
    Nil
}

pub type EvaCache = Vec<EvaCached>;

fn isinteger(s: &str) -> bool {
    s.parse::<isize>().is_ok() && !s.contains('.')
}

fn isfloat(s: &str) -> bool {
    s.parse::<f64>().is_ok() && s.contains('.')
}

pub fn isnumeric(token: &str) -> bool {
    isinteger(token) || isfloat(token)
}
