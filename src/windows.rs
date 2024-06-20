use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Globalization::GetUserDefaultLocaleName;
use windows::Win32::System::SystemServices::LOCALE_NAME_MAX_LENGTH;

pub fn lang_windows() -> Option<String> {
    unsafe {
        let mut buffer: [u16; LOCALE_NAME_MAX_LENGTH as usize] =
            [0; LOCALE_NAME_MAX_LENGTH as usize];
        if GetUserDefaultLocaleName(&mut buffer) != 0 {
            let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
            let locale_name = OsString::from_wide(&buffer[..len]);
            if let Ok(result) = locale_name.into_string() {
                return Some(result);
            }
        }
    }
    None
}
