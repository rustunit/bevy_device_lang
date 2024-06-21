/// returns first language from `[NSLocale preferredLanguages]`
/// (see [docs](https://developer.apple.com/documentation/foundation/nslocale/1415614-preferredlanguages))
pub fn lang_apple() -> Option<String> {
    use icrate::Foundation::NSString;
    use objc2::{class, msg_send, runtime::NSObject};

    let nslocale = class!(NSLocale);
    let id = unsafe {
        let array: *const NSObject = msg_send![nslocale, preferredLanguages];
        let id: *const NSString = msg_send![array, objectAtIndex:0_isize];
        id.as_ref()?
    };

    Some(id.to_string())
}
