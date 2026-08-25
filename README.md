# trillium-nursery

[![ci][ci-badge]][ci]
[![crates.io version][version-badge]][crate]
[![docs.rs][docs-badge]][docs]
[![codecov][codecov-badge]][codecov]

[ci]: https://github.com/trillium-rs/trillium-nursery/actions?query=workflow%3ACI
[ci-badge]: https://github.com/trillium-rs/trillium-nursery/workflows/CI/badge.svg
[version-badge]: https://img.shields.io/crates/v/trillium-nursery.svg?style=flat-square
[crate]: https://crates.io/crates/trillium-nursery
[docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[docs]: https://docs.rs/trillium-nursery
[codecov-badge]: https://codecov.io/gh/trillium-rs/trillium-nursery/graph/badge.svg
[codecov]: https://codecov.io/gh/trillium-rs/trillium-nursery

Experimental combinators and wrappers for the [trillium.rs](https://trillium.rs) web framework.

Everything here is an api experiment: residents that prove themselves graduate
into the `trillium` crate or a dedicated crate, and the rest are removed. Minor
versions add, change, and delete freely — pin a version if you depend on one,
and expect the stable home of any given item to eventually be elsewhere.

Residents: `when` (run a handler only for conns matched by a predicate) and
`unhalt` (run a handler but prevent it from halting the conn).

## Example

```rust
use trillium_nursery::{unhalt, when};

let handler = (
    when(|conn: &trillium::Conn| conn.path() == "/teapot", "short and stout"),
    unhalt("this halting handler no longer stops the chain"),
    |conn: trillium::Conn| async move { conn.with_status(418) },
);
```

## Safety

This crate uses `#![forbid(unsafe_code)]`.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
