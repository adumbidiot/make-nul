use core::convert::TryInto;
use core::mem::MaybeUninit;
use std::ffi::OsString;
use std::os::raw::c_int;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use winapi::shared::lmcons::UNLEN;
use winapi::shared::minwindef::FALSE;
use winapi::shared::minwindef::MAX_PATH;
use winapi::shared::minwindef::TRUE;
use winapi::um::shlobj::SHGetSpecialFolderPathW;
use winapi::um::shlobj::CSIDL_DESKTOP;
use winapi::um::winbase::lstrlenW;
use winapi::um::winbase::GetUserNameW;

/// Get the user name of the current user.
pub fn get_user_name() -> std::io::Result<OsString> {
    const BUFFER_LEN: u32 = UNLEN + 1;

    let mut buffer_len = BUFFER_LEN;
    let mut buffer = MaybeUninit::<[u16; BUFFER_LEN as usize]>::uninit();

    // # Safety
    // This is safe as the buffer exists and the correct buffer length is passed to this function for initialization.
    let ret = unsafe { GetUserNameW(buffer.as_mut_ptr().cast(), &mut buffer_len) };

    if ret == 0 {
        return Err(std::io::Error::last_os_error());
    }

    // # Safety
    // The data must be valid at this point.
    // The length of data (minus the nul terminator) has been updated and is passed in.
    // There are only immutable references left to `buffer`, so making another immutable one is safe.
    let buffer = unsafe {
        let len = (buffer_len - 1) as usize;
        std::slice::from_raw_parts(buffer.as_ptr().cast(), len)
    };

    Ok(OsString::from_wide(buffer))
}

/// The location of a folder
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ConstantSpecialItemIdList {
    Desktop,
}

impl ConstantSpecialItemIdList {
    fn as_c_int(self) -> c_int {
        match self {
            Self::Desktop => CSIDL_DESKTOP,
        }
    }
}

pub fn get_special_folder_path(
    csidl: ConstantSpecialItemIdList,
    create_folder: bool,
) -> Option<PathBuf> {
    const BUFFER_LEN: usize = MAX_PATH + 1;

    let mut buffer = MaybeUninit::<[u16; BUFFER_LEN]>::uninit();
    let create_folder = if create_folder { TRUE } else { FALSE };

    // # Safety
    // The buffer exists and has a minimum length of MAX_PATH.
    let ret = unsafe {
        SHGetSpecialFolderPathW(
            std::ptr::null_mut(),
            buffer.as_mut_ptr().cast(),
            csidl.as_c_int(),
            create_folder,
        )
    };

    if ret != TRUE {
        return None;
    }

    // # Safety
    // The data must be valid at this point.
    // The data is NUL terminated.
    // There are only immutable references left to `buffer`, so making another immutable one is safe.
    let buffer = unsafe {
        let len: usize = lstrlenW(buffer.as_ptr().cast())
            .try_into()
            .expect("could not convert string length into a `usize`");
        std::slice::from_raw_parts(buffer.as_ptr().cast(), len)
    };

    Some(OsString::from_wide(buffer).into())
}
