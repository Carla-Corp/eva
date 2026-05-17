use uuid::Uuid;

pub enum CodeStatus {
    JustFailed = -1,
    Ok = 0,
    FailToOpenEvaFile = 1
}

pub type Error = String;

#[derive(Debug, Clone, Default)]
pub enum EvaValue {
    String(String),
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

pub type EvaCache = Vec<(Uuid, EvaCached)>;
