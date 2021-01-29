#[my_crate::hello]
fn wrapped_function() {}

#[test]
fn works() {
    wrapped_function();
}
