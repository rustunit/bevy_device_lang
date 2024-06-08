#[cfg(target_os = "android")]
mod android;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn get_lang() -> String {
    crate::apple::lang_apple()
}

#[cfg(target_os = "android")]
pub fn get_lang() -> String {
    crate::android::lang_android()
}

//TODO: support wasm
#[cfg(target_arch = "wasm32")]
pub fn get_lang() -> String {
    "en-US".into()
}

//TODO: support linux
#[cfg(target_os = "linux")]
pub fn get_lang() -> String {
    "en-US".into()
}

//TODO: support win
#[cfg(target_os = "windows")]
pub fn get_lang() -> String {
    "en-US".into()
}
