use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
pub enum EvaValueTag {
    String,
    Number,
    Bool,
    Nil,
}

#[repr(C)]
pub union EvaValueData {
    pub string: *const c_char,
    pub number: f64,
    pub boolean: bool,
}

#[repr(C)]
pub struct EvaValueFFI {
    pub tag: EvaValueTag,
    pub data: EvaValueData,
}

impl crate::core::EvaValue {
    pub fn to_ffi(&self) -> EvaValueFFI {
        match self {
            crate::core::EvaValue::String(s) => {
                let cstr = CString::new(s.clone()).unwrap();
                let ptr = cstr.into_raw();

                EvaValueFFI {
                    tag: EvaValueTag::String,
                    data: EvaValueData { string: ptr },
                }
            }

            crate::core::EvaValue::Number(i) => EvaValueFFI {
                tag: EvaValueTag::Number,
                data: EvaValueData { number: *i },
            },

            crate::core::EvaValue::Bool(b) => EvaValueFFI {
                tag: EvaValueTag::Bool,
                data: EvaValueData { boolean: *b },
            },

            crate::core::EvaValue::Nil => EvaValueFFI {
                tag: EvaValueTag::Nil,
                data: EvaValueData { boolean: false }
            },

            _ => todo!()
        }
    }
}
