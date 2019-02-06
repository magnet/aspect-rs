//! # An Aspect Toolkit for Rust
//! 
//! Aspect-RS is a project aiming to provide common ground for the main Aspect-Oriented use cases in Rust. By leveraging the trait system, declarative and procedural macros, Aspect-RS provides blocks that let you wrap methods with your custom logic.
//! 
//! The project has been extracted from the [Metered project](https://github.com/magnet/metered-rs), which uses the technique to build metrics that can work on expressions or methods, whether they're `async` or not. The technique seemed general enough to be in its own crate and see if it is of any interest to other parties.
//! 
//! Aspect-RS provides "pointcut" traits when entering or exiting an expression (`OnEnter` and `OnResult`), experimental `Update` and `UpdateRef` traits that can use parameter shadowing to intercept and update method parameters, and weaving constructs useful when building procedural macros. Please look at the [Metered project](https://github.com/magnet/metered-rs) to see Aspect-RS in action.


pub mod update;

pub trait Enter {
    type E;
    fn enter(&self) -> Self::E;
}

pub enum Advice {
    Return,
    Retry,
}

pub trait OnResult<R>: Enter {
    fn on_result(&self, _enter: <Self as Enter>::E, _result: &R) -> Advice {
        Advice::Return
    }
}

pub trait OnResultMut<R>: Enter {
    fn on_result(&self, _enter: <Self as Enter>::E, _result: &mut R) -> Advice {
        Advice::Return
    }
}

#[macro_export]
macro_rules! define {
    ($name:ident: $on_result:path) => {
        #[macro_export]
        macro_rules! $name {
            ($aspect:ident, $e:expr) => {{
                loop {
                    let _aspect = $aspect;
                    let _enter = $crate::Enter::enter(_aspect);
                    let _result = $e;
                    let _advice = $on_result(_aspect, _enter, &_result);
                    match _advice {
                        $crate::Advice::Return => break _result,
                        _ => continue,
                    }
                }
            }};
        }
    };
}
