//! # An Aspect Toolkit for Rust
//!
//! Aspect-RS is a project aiming to provide common ground for the main Aspect-Oriented use cases in Rust. By leveraging the trait system, declarative and procedural macros, Aspect-RS provides blocks that let you wrap methods with your custom logic.
//!
//! The project has been extracted from the [Metered project](https://github.com/magnet/metered-rs), which uses the technique to build metrics that can work on expressions or methods, whether they're `async` or not. The technique seemed general enough to be in its own crate and see if it is of any interest to other parties.
//!
//! Aspect-RS provides "pointcut" traits when entering or exiting an expression (`OnEnter` and `OnResult`), experimental `Update` and `UpdateRef` traits that can use parameter shadowing to intercept and update method parameters, and weaving constructs useful when building procedural macros. Please look at the [Metered project](https://github.com/magnet/metered-rs) to see Aspect-RS in action.

#![deny(missing_docs)]
#![deny(warnings)]

pub mod update;

/// The `Enter` trait is called when entering in an aspect, before the wrapped expression is called.
pub trait Enter {
    /// The type returned by the `enter` function and carried to `OnResult`
    type E;

    /// `enter` is called when entering in an aspect
    ///
    /// Use it to set-up some context before calling the expression. For instance, the `ResponseTime` metric in the [metered](https://github.com/magnet/metered-rs) crate uses it to get the current time before the invocation, and pass it over to `OnResult` to compute the elapsed time.
    ///
    /// Aspects which don't need enter can simply do nothing and return unit.
    fn enter(&self) -> Self::E;
}

/// An `Advice` describes what the aspect should do on return
pub enum Advice {
    /// Return the expression value
    Return,
    /// Call the expression again.
    ///
    /// Experimental. Use with caution regarding side-effects in the expression and possible loops.
    Retry,
}

/// The `OnResult` trait is implemented on Aspects to get notified when an expression has returned.
pub trait OnResult<R>: Enter {
    /// Called when an expression has returned.
    ///
    /// This function is passed both the enter return value, and the expression return value.
    ///
    /// `on_result` does not get a chance to alter the returned result. Use `OnResultMut` for that purpose.
    fn on_result(&self, _enter: <Self as Enter>::E, _result: &R) -> Advice {
        Advice::Return
    }
}

/// The `OnResult` trait is implemented on Aspects to get notified when an expression has returned, and provide
/// the possibility to alter the result.
pub trait OnResultMut<R>: Enter {
    /// Called when an expression has returned.
    ///
    /// This function is passed both the enter return value, and the expression return value.
    fn on_result(&self, _enter: <Self as Enter>::E, _result: &mut R) -> Advice {
        Advice::Return
    }
}
