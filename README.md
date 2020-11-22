# Pasts

#### Minimal and simpler alternative to the futures crate.

[![tests](https://github.com/Nezeky/pasts/workflows/tests/badge.svg)][2]
[![docs](https://docs.rs/pasts/badge.svg)][0]
[![crates.io](https://img.shields.io/crates/v/pasts.svg)][1]

[About][4] | [Source][5] | [Changelog][3] | [Tutorial][6]

# About
 - No required std (on no\_std, a single allocation is required)
 - No slow compiling proc macros (fast compile times)
 - No dependencies
 - No cost (True zero-cost abstractions!)
 - No pain (API super easy to learn & use!)
 - No unsafe code left for *you* to write for working with `Future`s (ability to
   `#[forbid(unsafe_code)]`)
 - No platform-specific API differences (code works everywhere!).

Check out the [documentation][0] for examples.

### Supported Platforms
Pasts targets all platforms that can run Rust.  The `exec!()` executor works
on at least the following platforms (may work on others):
 - All platforms that support threading (includes all tier 1 and some tier 2, 3)
 - Web Assembly In Browser (Tier 2)

## License
Licensed under either of
 - Apache License, Version 2.0,
   ([LICENSE-APACHE][7] or [https://www.apache.org/licenses/LICENSE-2.0][8])
 - Zlib License,
   ([LICENSE-ZLIB][9] or [https://opensource.org/licenses/Zlib][10])
at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[0]: https://docs.rs/pasts
[1]: https://crates.io/crates/pasts
[2]: https://github.com/Nezeky/pasts/actions?query=workflow%3Atests
[3]: https://github.com/Nezeky/pasts/blob/master/CHANGELOG.md
[4]: https://github.com/Nezeky/pasts/blob/master/README.md
[5]: https://github.com/Nezeky/pasts
[6]: https://aldaronlau.com/
[7]: https://github.com/Nezeky/pasts/blob/master/LICENSE-APACHE
[8]: https://www.apache.org/licenses/LICENSE-2.0
[9]: https://github.com/Nezeky/pasts/blob/master/LICENSE-ZLIB
[10]: https://opensource.org/licenses/Zlib
