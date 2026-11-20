# rs-to-readme

[![crates.io](https://img.shields.io/crates/v/rs-to-readme.svg)](https://crates.io/crates/rs-to-readme) [![docs.rs](https://docs.rs/rs-to-readme/badge.svg)](https://docs.rs/rs-to-readme) ![license](https://img.shields.io/badge/license-MIT-blue.svg) [![CI](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml/badge.svg)](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml)

📖 لغات أخرى: [日本語](README-Japan.md) / [English](README-English.md) / [中文](README-Chinese.md) /
[한국어](README-Korea.md) / [Español](README-Spain.md) / [Français](README-France.md) /
[Deutsch](README-Germany.md) / [Italiano](README-Italy.md) / [Русский](README-Russia.md) ·
للدمج في مشروع آخر، راجع **[PORTING.md](PORTING.md)**.

مكتبة + أداة CLI تُنشئ ملف README.md كامل لحزمة Rust انطلاقًا من بيانات
`Cargo.toml` الوصفية وتعليقات rustdoc.

**فلسفة التصميم**: يُعامَل الكود المصدري (بيانات `Cargo.toml` الوصفية،
تعليق التوثيق `//!` على مستوى الحزمة، وتعليق `///` لكل عنصر عام) بوصفه
المصدر الوحيد للحقيقة، ويُشتق ملف README.md منه آليًا. وهذا يمنع بنيويًا
المشكلة الشائعة "تغيّر الكود ونُسي تحديث README".

استخدم وضع `--check` في CI للتحقق من تطابق الناتج المُولَّد مع ملف
README.md الحالي، وبذلك يمكن رصد طلبات السحب التي غيّرت الكود ونسيت
إعادة توليد README آليًا.

## Installation

```bash
cargo install rs-to-readme
```

## API Overview

- `RsToReadmeError` (enum)
- `Result` (type)
- `generate_readme` (fn) — يُولّد محتوى README.md من المجلد الجذري للحزمة
  (حيث يوجد `Cargo.toml`). يحدد `entry_file` الملف المصدري الذي يُستخرج
  منه التوثيق (عادةً `src/lib.rs`، أو `src/main.rs` للحزم من نوع bin فقط).

## License

مرخّص بموجب MIT.
