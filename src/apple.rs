pub fn lang_apple() -> String {
    use icrate::Foundation::NSString;
    use objc2::{class, msg_send, runtime::NSObject};

    let nslocale = class!(NSLocale);
    let id = unsafe {
        let array: *const NSObject = msg_send![nslocale, preferredLanguages];
        let id: *const NSString = msg_send![array, objectAtIndex:0_isize];
        id.as_ref().unwrap()
    };

    id.to_string()
}
