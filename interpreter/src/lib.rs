use std::{ffi::CStr, fs};

pub mod core;
pub mod parser;
pub mod statics;
pub mod functions;

#[unsafe(no_mangle)]
extern "C" fn push_parser(path: *const i8) -> isize {
    let rs_path =
        unsafe { CStr::from_ptr(path) }
            .to_string_lossy()
            .to_string();

    let Ok(data) = fs::read_to_string(rs_path) else {
        return core::CodeStatus::FailToOpenEvaFile as isize;
    };

    let status = parser::Parser::new(&data)
        .parse();

    return core::CodeStatus::Ok as isize;
}
