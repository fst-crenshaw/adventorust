#[path = "../../share.rs"]
pub(crate) mod foo;      // including the constant.  Exporting the constant?
//pub(crate) mod share;

// Include the special constant from the share.
//use share::SPECIAL_CONSTANT;

pub fn public_function() {
    //println!("{:?}: `public_function()`", share::SPECIAL_CONSTANT);
    println!("I am using it now: {:?}.", SPECIAL_CONSTANT);
}

// Export the constant for our friends to use.
pub use foo::SPECIAL_CONSTANT;
