use std::ffi::{CStr, CString};
use SdlResult;
use get_error;
use util::CStringExt;
use libc::c_char;

use sys::filesystem as ll;

pub fn base_path() -> SdlResult<String> {
    let result = unsafe {
        let buf = ll::SDL_GetBasePath();
        String::from_utf8_lossy(CStr::from_ptr(buf as *const _).to_bytes()).to_string()
    };

    if result.len() == 0 {
        Err(get_error())
    } else {
        Ok(result)
    }
}

pub fn pref_path(org: &str, app: &str) -> SdlResult<String> {
    let result = unsafe {
        let org = try!(CString::new(org).unwrap_or_sdlresult());
        let app = try!(CString::new(app).unwrap_or_sdlresult());
        let buf = ll::SDL_GetPrefPath(org.as_ptr() as *const c_char, app.as_ptr() as *const c_char);
        String::from_utf8_lossy(CStr::from_ptr(buf as *const _).to_bytes()).to_string()
    };

    if result.len() == 0 {
        Err(get_error())
    } else {
        Ok(result)
    }
}
