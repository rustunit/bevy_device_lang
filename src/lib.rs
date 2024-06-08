#[cfg(target_os = "android")]
mod android;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple;

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

//TODO: support linux
#[cfg(target_os = "linux")]
pub fn get_lang() -> Option<String> {
    None
}

//TODO: support win
#[cfg(target_os = "windows")]
pub fn get_lang() -> Option<String> {
    None
}
