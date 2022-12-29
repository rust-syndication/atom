# Changelog

## 0.12.0 - Unreleased

- Wrap `quick_xml::XmlError` into a newtype [`#65`](https://github.com/rust-syndication/atom/pull/65)
- Implement `std::error::Error` for `XmlError`. Mark helper traits as `pub(crate)` to prevent their accidental leakage to public API [`#66`](https://github.com/rust-syndication/atom/pull/66)
- Bump MSRV (Minimum Supported Rust Version) from 1.40.0 to 1.54.0 [`#66`](https://github.com/rust-syndication/atom/pull/66) and [`#69`](https://github.com/rust-syndication/atom/pull/69)
- Upgrade `quick_xml` to `0.27` and `derive_builder` to `0.12` [`#67`](https://github.com/rust-syndication/atom/pull/67)

## 0.11.0 - 2021-10-20

- Disable clock feature of chrono to mitigate RUSTSEC-2020-0159 [`#57`](https://github.com/rust-syndication/atom/pull/57)
- Escape Content's value unless it contains xhtml [`#52`](https://github.com/rust-syndication/atom/pull/52)
- Preserve entities and open tags (e.g `&amp;`, `<br/>`) in xhtml content [`#53`](https://github.com/rust-syndication/atom/pull/53)
- Add support of xml:base and xml:land in a Feed [`#55`](https://github.com/rust-syndication/atom/pull/55)

## 0.10.0 - 2021-06-06

- Infallible builders [`13af228`](https://github.com/rust-syndication/atom/commit/13af228967934f6869886a42bd6427cd6d24da64)
- Introduce Text type to represent Atom Text constructs [`45cbd6b`](https://github.com/rust-syndication/atom/commit/45cbd6b61af57a4bcfc98600b5510139c75baf10)
- Rename text::ContentType to TextType [`fa32372`](https://github.com/rust-syndication/atom/commit/fa323721845b496b9264ab92937fa3f29ca11c1d)

## 0.9.1 - 2021-01-07

- update quick-xml version to 0.20 [`48caa33`](https://github.com/rust-syndication/atom/commit/48caa33be11f33ad543de29b2522c90766a5eaf9)

## 0.9.0 - 2020-05-13

- Update quick-xml to 0.18 [`#28`](https://github.com/rust-syndication/atom/pull/28)
- Diligently parse dates [`#23`](https://github.com/rust-syndication/atom/pull/23)
- Expose all fields of the model types [`#25`](https://github.com/rust-syndication/atom/pull/25)

## 0.8.0 - 2020-01-23

- Add a default builders feature that can be disabled [`#19`](https://github.com/rust-syndication/atom/pull/19)
- Handwrite Error conformance so thiserror is not needed [`#18`](https://github.com/rust-syndication/atom/pull/18)
- Use thiserror instead of failure [`#17`](https://github.com/rust-syndication/atom/pull/17)
- prepare for 0.8.0 release [`ce4c783`](https://github.com/rust-syndication/atom/commit/ce4c783d01edf8266456a49bf2c2a75da10d5b24)

## 0.7.0 - 2019-12-15

- update dependencies [`#16`](https://github.com/rust-syndication/atom/pull/16)
- add feature: with-serde [`#13`](https://github.com/rust-syndication/atom/pull/13)
- replace String with ::chrono::DateTime&lt;::chrono::FixedOffset&gt; [`#12`](https://github.com/rust-syndication/atom/pull/12)
- update edition: from 2015 to 2018 [`b74bb6a`](https://github.com/rust-syndication/atom/commit/b74bb6ac8243187008cb084c661592e7aa887426)

## 0.6.0 - 2018-05-29

- prevent mixed content warning on crates.io [`#8`](https://github.com/rust-syndication/atom/pull/8)
- update quick-xml and use failure crate for Error [`7d75966`](https://github.com/rust-syndication/atom/commit/7d759667cfae0f90b7449e09dbe08678aed47c0b)

## 0.5.8 - 2018-02-12

- Optionally implement Serialize/Deserialize for data structures (fixes #5) [`#5`](https://github.com/rust-syndication/atom/issues/5)

## 0.5.7 - 2018-01-27

- fix text extraction [`78d62f0`](https://github.com/rust-syndication/atom/commit/78d62f0527ee9a339b9016362fdd2e02688ef74d)

## 0.5.6 - 2017-11-27

- Update quick-xml to 0.10 [`9cf8d23`](https://github.com/rust-syndication/atom/commit/9cf8d23c55efd08a6bb5369ea64370f02b60cfeb)

## 0.5.5 - 2017-11-07

- content type needs to be xhtml for xhtml content [`#2`](https://github.com/rust-syndication/atom/pull/2)

## 0.5.4 - 2017-07-16

- Derive builders with Into&lt;T&gt; and default values [`e72b20a`](https://github.com/rust-syndication/atom/commit/e72b20aa259292b8c9e390252266a1b7057d42e0)

## 0.5.3 - 2017-07-02

- Write Atom namespace [`#1`](https://github.com/rust-syndication/atom/issues/1)
- Formatted with rustfmt-nightly 0.1.7 [`5a5812c`](https://github.com/rust-syndication/atom/commit/5a5812c9a504fef681a3bfe11c069e43655767c9)

## 0.5.2 - 2017-06-23

- Added examples for Extension methods [`b5acfe0`](https://github.com/rust-syndication/atom/commit/b5acfe0a21f556d0205279bb7160151b2a7e5823)
- Added builders using derive_builders, added missing extension setter methods [`33ddd21`](https://github.com/rust-syndication/atom/commit/33ddd21e55739b7c9a7c01203c028bc64d197878)

## 0.5.1 - 2017-06-23

- Fixed Cargo.toml category slug [ci skip] [`3711804`](https://github.com/rust-syndication/atom/commit/3711804ade9769b29ca698296337a334003bbb8f)

## 0.5.0 - 2017-06-23

- Added reading, tests [`9cb8e2b`](https://github.com/rust-syndication/atom/commit/9cb8e2be94a67aee2e9a69624ba8e7473ab83ad9)
- Added writing [`338c840`](https://github.com/rust-syndication/atom/commit/338c840ee780c8c9726a63171684a094e1d7ccf0)
- Added support for extensions [`825d782`](https://github.com/rust-syndication/atom/commit/825d7821a47dcddada177e1d37fe20a35786bd63)
