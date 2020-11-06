use lib::{public_function};

#[path = "../../share.rs"]
pub(crate) mod share;

fn main() {
    println!("Hello, world {:?}!", share::SPECIAL_CONSTANT);
    public_function();
}
