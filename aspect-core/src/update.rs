//! An experimental module providing a way to update method parameters in aspects.

/// The trait `Update` allows a value to be updated while guaranteeing the return type is the same.
pub trait Update<T> {
    /// Update a value
    fn update(_: T) -> T;
}

/// The trait `UpdateRef` allows a value to be updated by using its reference.
pub trait UpdateRef<T> {
    /// Update a value by reference
    fn update_ref(_: &mut T);
}

impl<T, U: UpdateRef<T>> Update<T> for U {
    fn update(mut t: T) -> T {
        <U as UpdateRef<_>>::update_ref(&mut t);
        t
    }
}


#[cfg(test)]
mod test {
    use super::{Update, UpdateRef};

    struct Doubler;

    impl<'a> Update<&'a mut u64> for Doubler {
        fn update(t: &'a mut u64) -> &'a mut u64 {
            Doubler::update_ref(t);
            t
        }
    }
    // Can also be implemented on reference types
    impl UpdateRef<u64> for Doubler {
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
            let foo = <Doubler as Update<_>>::update(foo);
            intercepted(foo)
        }

        fn _intercepted_ref(foo: &mut u64) {
            // Use shadowing to update a value and guarantee its type
            let foo = <Doubler as Update<_>>::update(foo);
            intercepted_ref(foo)
        }

        let mut v = 42;
        let r = _intercepted(v);
        assert_eq!(r, 86);

        _intercepted_ref(&mut v);
        assert_eq!(v, 86);
    }
}
