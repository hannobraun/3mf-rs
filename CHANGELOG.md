# Changelog

## v0.7.0 (2025-01-31)

- Update dependencies ([#58])
- **Breaking change:** Upgrade to `quick-xml` 0.37 ([#61])

[#58]: https://github.com/hannobraun/3mf-rs/pull/58
[#61]: https://github.com/hannobraun/3mf-rs/pull/61

## v0.6.0 (2024-09-18)

- Update dependencies ([#46], [#48], [#49], [#50], [#51], [#52])
- **Breaking change:** Fix crash when parsing `<metadatagroup>` ([#54])

[#46]: https://github.com/hannobraun/3mf-rs/pull/46
[#48]: https://github.com/hannobraun/3mf-rs/pull/48
[#49]: https://github.com/hannobraun/3mf-rs/pull/49
[#50]: https://github.com/hannobraun/3mf-rs/pull/50
[#51]: https://github.com/hannobraun/3mf-rs/pull/51
[#52]: https://github.com/hannobraun/3mf-rs/pull/52
[#54]: https://github.com/hannobraun/3mf-rs/pull/54

## v0.5.0 (2024-02-14)

- Add support for reading 3MF files ([#28], [#32])
- Don't require actual files when reading/writing 3MF ([#31])
- Accept `Into<Model>` in `write` ([#33])
- Update dependencies ([#34], [#36], [#37], [#39])
- Update README ([#40])

[#28]: https://github.com/hannobraun/3mf-rs/pull/28
[#31]: https://github.com/hannobraun/3mf-rs/pull/31
[#32]: https://github.com/hannobraun/3mf-rs/pull/32
[#33]: https://github.com/hannobraun/3mf-rs/pull/33
[#34]: https://github.com/hannobraun/3mf-rs/pull/34
[#36]: https://github.com/hannobraun/3mf-rs/pull/36
[#37]: https://github.com/hannobraun/3mf-rs/pull/37
[#39]: https://github.com/hannobraun/3mf-rs/pull/39
[#40]: https://github.com/hannobraun/3mf-rs/pull/40

## v0.4.0 (2023-02-17)

- Switch to Serde for writing XML ([#22])

[#22]: https://github.com/hannobraun/3mf-rs/pull/22

## v0.3.1 (2022-05-24)

- Remove unused bzip2 dependency ([#12], [#13])

[#12]: https://github.com/hannobraun/3mf-rs/pull/12
[#13]: https://github.com/hannobraun/3mf-rs/pull/13

## v0.3.0 (2022-04-13)

- Re-export `write::Error` from root module ([#9])
- Accept `&Path` instead of `PathBuf` in `write` [#10]

[#9]: https://github.com/hannobraun/3mf-rs/pull/9
[#10]: https://github.com/hannobraun/3mf-rs/pull/10

## v0.2.0 (2021-11-20)

- Use `f64` to represent numbers ([#6])

[#6]: https://github.com/hannobraun/3mf-rs/pull/6

## v0.1.0 (2021-10-24)

Initial release.
