# Thierry Berger Game Analytics

My crate to handle [Game Analytics](gameanalytics.com) from Rust, for now focused on iOS.

<!-- TODO: Demo -->

## Features



## Instructions

1. iOS
   1. Add to XCode: Add SPM (Swift Package Manager) dependency
   2. iOS: Add `GA_GAME_KEY` and `GA_SECRET_KEY` to your Info.plist
   3. Add iOS Game Analytics official SDK Swift dependency, and configure it as they recommend (build phases, linker, header file...)
2. Add Rust dependency

### 1. Add to XCode

* Go to `File` -> `Add Package Dependencies` and paste `https://github.com/ThierryBerger/tb_ga.git` into the search bar on the top right.

### 2. Add Rust dependency

```sh
cargo add tb_ga
```

or

```toml
# always pin to the same exact version you also of the Swift package
tb_ga = { version = "=0.1.0" }
```

## Troubleshooting

* File an issue!

## Dev notes

### Resources

- https://docs.gameanalytics.com/event-tracking-and-integrations/sdks-and-collection-api/platform-sdks/ios
- https://shadowfacts.net/2023/rust-swift/

## License

All code in this repository is dual-licensed under either:

* MIT License (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
* Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option. This means you can select the license you prefer.

## Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
