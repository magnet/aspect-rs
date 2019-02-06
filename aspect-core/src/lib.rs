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
