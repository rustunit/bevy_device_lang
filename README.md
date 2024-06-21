# bevy_device_lang

[![crates.io][sh_crates]][lk_crates]
[![docs.rs][sh_docs]][lk_docs]
[![discord][sh_discord]][lk_discord]

[sh_crates]: https://img.shields.io/crates/v/bevy_device_lang.svg
[lk_crates]: https://crates.io/crates/bevy_device_lang
[sh_docs]: https://img.shields.io/docsrs/bevy_device_lang
[lk_docs]: https://docs.rs/bevy_device_lang/latest/bevy_device_lang/
[sh_discord]: https://img.shields.io/discord/1176858176897953872?label=discord&color=5561E6
[lk_discord]: https://discord.gg/rQNeEnMhus

Provides access device language cross-platform: iOS, Android, Web (Wasm), Windows & Linux.
Useful to support app localization in the right language.

See also:
[bevy_ios_iap](https://github.com/rustunit/bevy_ios_iap), [bevy_ios_gamecenter](https://github.com/rustunit/bevy_ios_gamecenter), [bevy_ios_notifications](https://github.com/rustunit/bevy_ios_notifications), [bevy_ios_alerts](https://github.com/rustunit/bevy_ios_alerts), [bevy_ios_review](https://github.com/rustunit/bevy_ios_review) & [bevy_ios_impact](https://github.com/rustunit/bevy_ios_impact)

## Features
* macOS, iOS (using `[NSLocale preferredLanguage]` see [docs](https://developer.apple.com/documentation/foundation/nslocale/1415614-preferredlanguages))

* Android (using `Locale.getDefault().toLanguageTag` see [docs](https://developer.android.com/reference/java/util/Locale#toLanguageTag()))
* Wasm (uses `web-sys` and `Navigator.language()` see [docs](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/language))
* Windows (using `GetUserDefaultLocaleName` see [docs](https://learn.microsoft.com/en-us/windows/win32/api/winnls/nf-winnls-getuserdefaultlocalename))
* Linux (using Env: `LC_ALL`, `LC_MESSAGES` or `LANG`)

## Usage

Add dependency: `bevy_device_lang = "0.4"`

```rust
fn bevy_system() {
    let lang : Option<String> = bevy_device_lang::get_lang();
}
```

# License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

## Your contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
