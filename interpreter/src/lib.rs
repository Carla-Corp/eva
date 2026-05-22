use std::{ffi::CStr, fs, os::raw::c_int};

use crate::{core::EvaValue, ffi::*, parser::Parser};

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

    let Ok(data) = fs::read_to_string(&rs_path) else {
        return Box::into_raw(Box::new(EvaParser {
            status: core::CodeStatus::FailToOpenEvaFile as isize,
            parser: std::ptr::null_mut(),
        }));
    };

    let filename =
        std::path::Path::new(&rs_path)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

    let mut parser = parser::Parser::new(&filename, &data);
    if let Some(err) = parser.parse() {
        println!("{}", err);
        return Box::into_raw(Box::new(EvaParser {
            status: core::CodeStatus::JustFailed as isize,
            parser: Box::into_raw(Box::new(
                parser
            )),
        }));
    };

    return Box::into_raw(Box::new(EvaParser {
        status: core::CodeStatus::Ok as isize,
        parser: Box::into_raw(Box::new(
            parser
        )),
    }));
}

#[unsafe(no_mangle)]
extern "C" fn eva_check_exist_field_in_namespace(parser: *mut EvaParser, ns: *const i8, f: *const i8) -> bool {
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
extern "C" fn eva_get_list_length(list: EvaValueFFI) -> c_int {
    if list.tag != EvaValueTag::List {
        return 0;
    }

    let internal = unsafe { list.data.list } as *const Vec<EvaValue>;
    return unsafe { (*internal).len() } as c_int;
}

#[unsafe(no_mangle)]
extern "C" fn eva_get_map_length(map: EvaValueFFI) -> c_int {
    if map.tag != EvaValueTag::Map {
        return 0;
    }

    let internal = unsafe { map.data.map } as *const Vec<(String, EvaValue)>;
    return unsafe { (*internal).len() } as c_int;
}

#[unsafe(no_mangle)]
extern "C" fn eva_get_map_field(map: EvaValueFFI, f: *const i8) -> EvaValueFFI {
    let internal = unsafe { map.data.map } as *const Vec<(String, EvaValue)>;

    let field =
        unsafe { CStr::from_ptr(f) }
            .to_string_lossy()
            .to_string();

    unsafe {
        (&*internal)
            .iter()
            .find(|(k, _)| k == &field)
            .cloned()
            .map(|(_, v)| v.to_ffi())
            .unwrap_or_else(|| EvaValue::Nil.to_ffi())
    }
}

#[unsafe(no_mangle)]
extern "C" fn eva_get_all_keys_from_map(map: EvaValueFFI) -> EvaValueFFI {
    let internal = unsafe { map.data.map } as *const Vec<(String, EvaValue)>;

    let list: Vec<_> = unsafe {
        (&*internal)
            .iter()
            .map(|(k, _)| EvaValue::String(k.clone()))
            .collect()
    };

    EvaValue::List(list).to_ffi()
}

#[unsafe(no_mangle)]
extern "C" fn eva_dump_pointer(value: EvaValueFFI) {
    match value.tag {
        EvaValueTag::List => unsafe { _ = Box::from_raw(value.data.list); },
        EvaValueTag::Map => unsafe { _ = Box::from_raw(value.data.map); },
        EvaValueTag::String => unsafe { _ = Box::from_raw(value.data.string); },
        _ => {}
    }
}


#[unsafe(no_mangle)]
extern "C" fn eva_check_exist_field_in_map(map: EvaValueFFI, f: *const i8) -> bool {
    let internal = unsafe { map.data.map } as *const Vec<(String, EvaValue)>;

    let field =
        unsafe { CStr::from_ptr(f) }
            .to_string_lossy()
            .to_string();

    unsafe {
        (&*internal)
            .iter()
            .any(|(k, _)| k == &field)
    }
}

#[unsafe(no_mangle)]
extern "C" fn eva_get_list_field(list: EvaValueFFI, index: c_int) -> EvaValueFFI {
    let internal = unsafe { list.data.list } as *const Vec<EvaValue>;

    unsafe {
        let value =
            (&*internal)
                .get(index as usize)
                .cloned()
                .unwrap_or_default();

        value.to_ffi()
    }
}

#[unsafe(no_mangle)]
extern "C" fn eva_print_value(parser: *mut EvaParser, ns: *const i8, f: *const u8) {
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
