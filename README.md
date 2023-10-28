### 🚨WARNING: WORK IN PROGRESS🚨
This crate is a work in progress and version changes may introduce breaking changes without warning.

# libliquidfun-sys
A thin Rust ffi wrapper on top of Box2D (version 2.4.1) and LiquidFun. Built using [autocxx](https://github.com/google/autocxx).

This crate is used by [bevy_liquidfun](https://github.com/mmatvein/bevy_liquidfun) to integrate Box2D and LiquidFun with the [bevy game engine](https://github.com/bevyengine/bevy).

### Remarks
- A forked version of Box2D included as git subtree under `./box2d/`
- Additions to the automatically generated wrappers are under `./include/extras.hpp`
- The autocxx generated wrappers for all the joint definitions have some misalignment, possibly caused by the inheritance hierarchy. This is worked around by having factory methods for joint creation with all the parameters.

### Acknowledgements
This small wrapper stands on the shoulders of giants. Big thanks go to the original libraries, which do the heavy lifting:

[Box2D](https://github.com/erincatto/box2d) by Erin Catto

[LiquidFun](https://github.com/google/liquidfun) by Google

[autocxx](https://github.com/google/autocxx) by Google

Additionally, the version of Box2D used is a fork of Box2D:

[LiquidFun rebase onto newer Box2D](https://github.com/Birch-san/box2d/tree/liquidfun-rebase) by Birch-san

### License

This work is licensed under either of [Apache License, Version 2.0](https://github.com/mmatvein/libliquidfun-sys/blob/main/LICENSE-APACHE) or [MIT license](https://github.com/mmatvein/libliquidfun-sys/blob/main/LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.