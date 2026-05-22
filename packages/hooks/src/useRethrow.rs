```rust
use std::error;

pub trait FnReturnOrVoid<T, E = error::Error> {
    fn call(self, args: T) -> Result<(), E>;
}

impl<F> FnReturnOrVoid<(), F> for F {
    fn call(self, _: ()) -> Result<(), F::Output> {
        self()
    }
}

pub type FnWithArgs<Args, Ret> = Box<dyn Fn(&mut (), Args) -> Result<Ret, error::Error>>;

#[derive(Debug)]
pub struct ErrorWrapper<F: FnReturnOrVoid<E>, E = error::Error>(F);

impl<F: FnReturnOrVoid<E>, E> Fn withArgs<(), ()> for ErrorWrapper<F, E> {
    fn call(&mut self, args: ()) -> Result<(), E> {
        (self.0)(args)
    }
}

pub fn use_rethrow<Args, Ret>(
    fn_to_wrap: FnWithArgs<Args, Ret>,
) -> Box<dyn FnReturnOrVoid<Args, Ret>> {
    let mut error = None;
    Box::new(move |args: Args| {
        match (fn_to_wrap)(args) {
            Ok(_) => Ok(()),
            Err(err) => {
                // throwing within a setError happens within a react render context
                error = Some(err);
                Err(())
            }
        }
    })
}
```