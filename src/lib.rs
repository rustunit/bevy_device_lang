#[cfg(target_os = "android")]
mod android;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple;

#[cfg(target_os = "windows")]
mod windows;

/// return language code like "en-US", "en-UK" or two letter ones like "en", "de"
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn get_lang() -> Option<String> {
    crate::apple::lang_apple()
}

#[cfg(target_os = "android")]
pub fn get_lang() -> Option<String> {
    crate::android::lang_android()
}

#[cfg(target_family = "wasm")]
pub fn get_lang() -> Option<String> {
    web_sys::window().and_then(|w| w.navigator().language())
}

#[cfg(target_os = "linux")]
pub fn get_lang() -> Option<String> {
    if let Ok(value) = std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_MESSAGES"))
        .or_else(|_| std::env::var("LANG"))
    {
        Some(value.split('.').next().unwrap_or(&value).into())
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
pub fn get_lang() -> Option<String> {
    crate::windows::lang_windows()
}
