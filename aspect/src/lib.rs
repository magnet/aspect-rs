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
pub use aspect_weave::Weave;

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
    fn on_result(&self, enter: <Self as Enter>::E, _result: &R) -> Advice {
        self.leave_scope(enter)
    }

    /// Called when an expression has exited, but the return value isn't known.
    /// This can happen because of a panic, or if control flow bypasses a macro.
    /// This is also called by the default implementation of `on_result`.
    fn leave_scope(&self, _enter: <Self as Enter>::E) -> Advice {
        Advice::Return
    }
}

/// The `OnResultMut` trait is implemented on Aspects to get notified
/// when an expression has returned, and provide the possibility to
/// replace the result.
pub trait OnResultMut<R>: Enter {
    /// Called when an expression has returned.
    ///
    /// This function is passed both the enter return value, and the expression return value.
    fn on_result(&self, enter: <Self as Enter>::E, result: R) -> (Advice, R) {
        let advice = self.leave_scope(enter);
        (advice, result)
    }

    /// Called when an expression has exited, but the return value isn't known.
    /// This can happen because of a panic, or if control flow bypasses a macro.
    /// This is also called by the default implementation of `on_result`.
    fn leave_scope(&self, _enter: <Self as Enter>::E) -> Advice {
        Advice::Return
    }
}

impl<R, A: OnResult<R>> OnResultMut<R> for A {
    fn on_result(&self, enter: <Self as Enter>::E, result: R) -> (Advice, R) {
        let advice = <Self as OnResult<R>>::on_result(self, enter, &result);
        (advice, result)
    }

    fn leave_scope(&self, enter: <Self as Enter>::E) -> Advice {
        <Self as OnResult<R>>::leave_scope(self, enter)
    }
}
