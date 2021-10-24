# 3MF (3D Manufacturing Format) support for Rust [![crates.io](https://img.shields.io/crates/v/threemf.svg)](https://crates.io/crates/threemf) [![CI](https://github.com/hannobraun/3mf-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/hannobraun/3mf-rs/actions/workflows/ci.yml)

## About

This library provides support for [3MF] files to programs written in the Rust programming language. 3MF is a file format commonly used for 3D printing. It is typically exported from a CAD program, and imported to a slicer.

So far, functionality is limited to writing 3MF files, and only the most basic features of 3MF are supported. Adding support for reading 3MF files, and for more features of the 3MF format is very desirable, and any contributions toward that are very welcome.

[3MF]: https://en.wikipedia.org/wiki/3D_Manufacturing_Format


## Status

Functionality is limited, but what is currently there seems to work well. This library is used by (and has been extracted from) [Fornjot].

[Fornjot]: https://github.com/hannobraun/fornjot


## License

This project is open source software, licensed under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with the software, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for all details.

[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: https://github.com/hannobraun/3mf-rs/blob/main/LICENSE.md
