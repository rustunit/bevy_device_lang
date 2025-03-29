/// returns first language from `[NSLocale preferredLanguages]`
/// (see [docs](https://developer.apple.com/documentation/foundation/nslocale/1415614-preferredlanguages))
pub fn lang_apple() -> Option<String> {
    use objc2_foundation::NSLocale;

    let lang = unsafe { NSLocale::preferredLanguages().iter().next()?.to_string() };

    Some(lang.to_string())
}
