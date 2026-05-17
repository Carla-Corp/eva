use std::ffi::CString;
use std::os::raw::c_char;

use crate::core::EvaValue;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum EvaValueTag {
    String,
    Number,
    Bool,
    Map,
    List,
    Nil,
}

#[repr(C)]
pub union EvaValueData {
    pub string: *mut c_char,
    pub list: *mut Vec<EvaValue>,
    pub map: *mut Vec<(String, EvaValue)>,
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

            crate::core::EvaValue::Map(map) => {
                EvaValueFFI {
                    tag: EvaValueTag::Map,
                    data: EvaValueData {
                        map: Box::into_raw(Box::new(
                            map.clone()
                        ))
                    },
                }
            },

            crate::core::EvaValue::List(list) => {
                EvaValueFFI {
                    tag: EvaValueTag::List,
                    data: EvaValueData {
                        list: Box::into_raw(Box::new(
                            list.clone()
                        ))
                    },
                }
            },

            _ => todo!()
        }
    }
}
