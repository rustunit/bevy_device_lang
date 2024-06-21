use jni::{self, objects::JString};

/// uses `Locale.getDefault().toLanguageTag()` [docs](https://developer.android.com/reference/java/util/Locale#toLanguageTag())
pub fn lang_android() -> Option<String> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.ok()?;
    let mut env = vm.attach_current_thread().ok()?;
    let class = env.find_class("java/util/Locale").ok()?;
    let locale = env
        .call_static_method(class, "getDefault", "()Ljava/util/Locale;", &[])
        .ok()?
        .l()
        .ok()?;

    let lang: JString = env
        .call_method(locale, "toLanguageTag", "()Ljava/lang/String;", &[])
        .ok()?
        .l()
        .ok()?
        .into();

    let lang = env.get_string(&lang).ok()?;
    let lang: String = lang.into();
    Some(lang)
}
