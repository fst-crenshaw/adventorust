#[path = "../../share.rs"]
pub(crate) mod share;

pub fn public_function() {
    println!("{:?}: `public_function()`", share::SPECIAL_CONSTANT);
}
