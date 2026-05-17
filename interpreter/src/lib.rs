use std::{ffi::CStr, fs};

use crate::{core::EvaValue, ffi::{EvaValueData, EvaValueFFI, EvaValueTag}, parser::Parser};

pub mod core;
pub mod parser;
pub mod statics;
pub mod functions;
pub mod ffi;

#[repr(C)]
struct EvaParser {
    status: isize,
    parser: *const Parser,
}

#[unsafe(no_mangle)]
extern "C" fn eva_make_parser(path: *const i8) -> *mut EvaParser {
    let rs_path =
        unsafe { CStr::from_ptr(path) }
            .to_string_lossy()
            .to_string();

    let Ok(data) = fs::read_to_string(rs_path) else {
        return Box::into_raw(Box::new(EvaParser {
            status: core::CodeStatus::FailToOpenEvaFile as isize,
            parser: std::ptr::null_mut(),
        }));
    };

    let mut parser = parser::Parser::new(&data);
    _ = parser.parse();

    return Box::into_raw(Box::new(EvaParser {
        status: core::CodeStatus::Ok as isize,
        parser: Box::into_raw(Box::new(
            parser
        )),
    }));
}

#[unsafe(no_mangle)]
extern "C" fn eva_check_exist(parser: *mut EvaParser, ns: *const i8, f: *const i8) -> bool {
    let parser = unsafe { &mut *parser };
    let internal = unsafe { &*parser.parser };

    let namespace =
        unsafe { CStr::from_ptr(ns) }
            .to_string_lossy()
            .to_string();

    let field =
        unsafe { CStr::from_ptr(f) }
            .to_string_lossy()
            .to_string();

    for data in internal.cache.iter() {
        match data {
            core::EvaCached::Field(ns, field_name, _) => {
                if ns == &namespace && field_name == &field {
                    return true;
                }
            }
            _ => {}
        }
    }

    false
}

#[unsafe(no_mangle)]
extern "C" fn eva_get_value_from_namespace(parser: *mut EvaParser, ns: *const i8, f: *const i8) -> EvaValueFFI {
    let parser = unsafe { &mut *parser };
    let internal = unsafe { &*parser.parser };

    let namespace =
        unsafe { CStr::from_ptr(ns) }
            .to_string_lossy()
            .to_string();

    let field =
        unsafe { CStr::from_ptr(f) }
            .to_string_lossy()
            .to_string();

    for data in internal.cache.iter() {
        match data {
            core::EvaCached::Field(ns, field_name, value) => {
                if ns == &namespace && field_name == &field {
                    return value.to_ffi();
                }
            }
            _ => {}
        }
    }

    EvaValue::Nil.to_ffi()
}

#[unsafe(no_mangle)]
extern "C" fn eva_print_value(parser: *mut EvaParser, ns: *const i8, f: *const i8) {
    let parser = unsafe { &mut *parser };
    let internal = unsafe { &*parser.parser };

    let namespace =
        unsafe { CStr::from_ptr(ns) }
            .to_string_lossy()
            .to_string();

    let field =
        unsafe { CStr::from_ptr(f) }
            .to_string_lossy()
            .to_string();

    for data in internal.cache.iter() {
        match data {
            core::EvaCached::Field(ns, field_name, value) => {
                if ns == &namespace && field_name == &field {
                    println!("{:?}", value);
                }
            }
            _ => {}
        }
    }
}
