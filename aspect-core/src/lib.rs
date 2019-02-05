pub trait Update<T> {
    fn update(_: T) -> T;
}

pub trait UpdateRef<T> {
    fn update_ref(_: &mut T);
}

impl<T, U: UpdateRef<T>> crate::Update<T> for U {
    fn update(mut t: T) -> T {
        <U as crate::UpdateRef<_>>::update_ref(&mut t);
        t
    }
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

pub trait Enter {
    type E;
    fn enter(&self) -> Self::E;
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
                        _ => continue
                    }                             
                }
            }};
        }
    };
}

#[cfg(test)]
mod test {

    use crate::*;

    struct Doubler;

    impl<'a> crate::Update<&'a mut u64> for Doubler {
        fn update(t: &'a mut u64) -> &'a mut u64 {
            Doubler::update_ref(t);
            t
        }
    }
    // Can also be implemented on reference types
    impl crate::UpdateRef<u64> for Doubler {
        fn update_ref(t: &mut u64) {
            (*t) *= 2;
        }
    }

    fn intercepted(foo: u64) -> u64 {
        foo + 2
    }

    fn intercepted_ref(foo: &mut u64) {
        (*foo) += 2
    }

    #[test]
    fn test() {
        fn _intercepted(foo: u64) -> u64 {
            // Use shadowing to update a value and guarantee its type
            let foo = <Doubler as crate::Update<_>>::update(foo);
            intercepted(foo)
        }

        fn _intercepted_ref(foo: &mut u64) {
            // Use shadowing to update a value and guarantee its type
            let foo = <Doubler as crate::Update<_>>::update(foo);
            intercepted_ref(foo)
        }

        let mut v = 42;
        let r = _intercepted(v);
        assert_eq!(r, 86);

        _intercepted_ref(&mut v);
        assert_eq!(v, 86);
    }
}
