use std::marker::PhantomData;

/// A mock of a resource handle.
struct Handle<'a> {
    _phantom: PhantomData<&'a mut ()>,
}

/// A trait that allows us to acquire a `Handle`, using the lifetime of `self`
/// as the lifetime argument.
trait T {
    fn get_handle_via_trait(&self) -> Handle<'_> {
        Handle {
            _phantom: PhantomData,
        }
    }
}

/// A struct that implements our trait `T` and exposes a method that mutably
/// borrows `self`.
struct S;

impl S {
    fn borrow_mut(&mut self) { todo!() }
}

impl T for S {}

fn main() {
    let mut s = S;
    let h = s.get_handle_via_trait();
    // This won't compile, via error `E0502`, aka our old friend:
    // "cannot borrow as mutable because it is also borrowed as immutable."
    s.borrow_mut();
    drop(h);
    // After `h` is dropped however, we _can_ borrow `s` mutably!
    s.borrow_mut();
}

