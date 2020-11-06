#[path = "../../share.rs"]
pub(in super) mod share;
//pub(crate) mod share;

pub fn public_function() {
    println!("{:?}: `public_function()`", share::SPECIAL_CONSTANT);
}
