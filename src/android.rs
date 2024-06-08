use jni::{self, objects::JString};

pub fn lang_android() -> String {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    // let context = unsafe { JObject::from_raw(ctx.context().cast()) };
    let mut env = vm.attach_current_thread().unwrap();
    let class = env.find_class("java/util/Locale").unwrap();
    let locale = env
        .call_static_method(class, "getDefault", "()Ljava/util/Locale;", &[])
        .unwrap()
        .l()
        .unwrap();

    let lang: JString = env
        .call_method(locale, "toLanguageTag", "()Ljava/lang/String;", &[])
        .unwrap()
        .l()
        .unwrap()
        .into();

    let lang = env.get_string(&lang).unwrap();
    let lang: String = lang.into();
    lang
}
