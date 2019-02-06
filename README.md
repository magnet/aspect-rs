# aspect-rs
## An Aspect Toolkit for Rust

Aspect-RS is a project aiming to provide common ground for the main Aspect-Oriented use cases in Rust. By leveraging the trait system, declarative and procedural macros, Aspect-RS provides blocks that let you wrap methods with your custom logic.

The project has been extracted from the [Metered project](https://github.com/magnet/metered-rs), which uses the technique to build metrics that can work on expressions or methods, whether they're `async` or not. The technique seemed general enough to be in its own crate and see if it is of any interest to other parties.

Aspect-RS provides "pointcut" traits when entering or exiting an expression (`OnEnter` and `OnResult`), experimental `Update` and `UpdateRef` traits that can use parameter shadowing to intercept and update method parameters, and weaving constructs useful when building procedural macros. Please look at the [Metered project](https://github.com/magnet/metered-rs) to see Aspect-RS in action.


## Required Rust version

Aspect-RS works on `Rust` stable.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.