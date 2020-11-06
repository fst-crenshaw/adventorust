use lib::{public_function};

#[path = "../../share.rs"]
// Question: Why is this a mod here?  Why not use?
// Question: If we'd like to share SPECIAL CONSTANT to the outside world, how do we do that?

// The crate in pub(crate) is like scoping the constant to the crate.  One can also supply
// `super` and `self` to provide different levels of scope.
pub(crate) mod share;


fn main() {
    println!("Hello, world {:?}!", share::SPECIAL_CONSTANT);
    public_function();
}
